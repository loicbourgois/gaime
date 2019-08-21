import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router'
import { Game } from '../game/game';
import { GameService } from '../game/game.service';
import { User } from '../user/user';
import { UserService } from '../user/user.service';

declare global {
    interface Window { Gaime: any; }
}

@Component({
    selector: 'app-play',
    templateUrl: './play.component.html',
    styleUrls: ['./play.component.scss']
})
export class PlayComponent implements OnInit {

    game: Game = new Game();
    user: User = new User();
    websocket;
    username;
    jwt;
    onDataReceived;
    doRender;
    logs = '';
    initialisationScript = `
const Gaime = window.Gaime;

// Gaime.log('Initialisation start');

Gaime.canvas = document.getElementById('canvas');
Gaime.context = Gaime.canvas.getContext('2d');
Gaime.context.clearRect(0, 0, Gaime.canvas.width, Gaime.canvas.height);
if (!Gaime.snakeKeydownEventListener) {
    Gaime.snakeKeydownEventListener = true;
    document.addEventListener('keydown', function(e) {
        switch (e.keyCode) {
            case 37:
                Gaime.sendGameCommand('Left');
                break;
            case 38:
                Gaime.sendGameCommand('Up');
                break;
            case 39:
                Gaime.sendGameCommand('Right');
                break;
            case 40:
                Gaime.sendGameCommand('Down');
                break;
        }
    });
}

// Gaime.log('Initialisation end');
    `;
    renderScript = `
const Gaime = window.Gaime;

const canvas = Gaime.canvas;
const context = Gaime.context;

if (!Gaime.GameState) {
    return;
}

const cell_width = canvas.width / Gaime.GameState.width;
const cell_height = canvas.height / Gaime.GameState.height;
const border = 2;
const width = cell_width - border*2;
const height = cell_height - border*2;

context.fillStyle = '#eee';
for (let i = 0 ; i < Gaime.GameState.width ; i+=1) {
    for (let j = 0 ; j < Gaime.GameState.height ; j+=1) {
        const x = cell_width * i + border;
        const y = cell_height * j - border;
        context.fillRect(x, y, width, height);
    }
}

for (let key in Gaime.GameState.snakes ) {
    const snake = Gaime.GameState.snakes[key];
    if (snake.username === Gaime.username) {
        context.fillStyle = '#00f8';
    } else {
        context.fillStyle = '#f008';
    }
    snake.body_parts.forEach( body_part => {
        const x = cell_width * body_part.x + border;
        const y = cell_height * body_part.y - border;
        context.fillRect(x, y, width, height);
    });
    if (snake.is_alive === false) {
        context.fillStyle = '#0004';
        snake.body_parts.forEach( body_part => {
            const x = cell_width * body_part.x + border;
            const y = cell_height * body_part.y - border;
            context.fillRect(x, y, width, height);
        });
    } else {
        // Do nothing
    }
}

context.fillStyle = '#0f08';
Gaime.GameState.foods.forEach( food => {
    const x = cell_width * food.x + border;
    const y = cell_height * food.y - border;
    context.fillRect(x, y, width, height);
});
    `;
    onDataReceivedScript = `
const Gaime = window.Gaime;

// Gaime.log(JSON.stringify(Gaime.GameState, null, 4));
    `;

    constructor(
        private route: ActivatedRoute,
        private gameService: GameService,
        private userService: UserService
    ) {
    }

    async ngOnInit() {
        this.getGame();
        this.getUser();
    }

    getGame() {
        const gameId = this.route.snapshot.paramMap.get('gameId');
        this.gameService.getGame(gameId)
            .subscribe((data: Game) => {
                this.game = data;
            });
    }

    findGame() {
        if (!window.Gaime) {
            window.Gaime = {};
        } else {
            window.Gaime.GameState = null;
        }
        document.getElementById('find-game-button').focus();
        document.getElementById('find-game-button').blur();
        const game_websocket_url: string = this.game.websocket_url;
        this.username = this.user.username;
        this.jwt = this.userService.getJwt();
        const game_string_id = this.game.string_id;
        if (!game_websocket_url) {
            console.error('game_websocket_url', game_websocket_url);
            return;
        }
        if (!this.username) {
            console.error('username', this.username);
            return;
        }
        if (!this.jwt) {
            console.error('jwt', this.jwt);
            return;
        }
        if (!game_string_id) {
            console.error('game_string_id', game_string_id);
            return;
        }
        this.websocket = new WebSocket(game_websocket_url);
        this.websocket.addEventListener('error', event => {
            this.error(`Could not connect to websocket ${game_websocket_url}`);
        });
        this.websocket.addEventListener('open', event => {
            this.disableTextareas();
            this.disableFindGameButton();
            this.enableQuitButton();
            this.compileScripts();
            this.requestFindGame(this.username, this.jwt, game_string_id);
        });
        this.websocket.addEventListener('message', (event) => {
            let jsonData: any;
            try {
                jsonData = JSON.parse(event.data);
            } catch (error) {
                // console.warn(error);
            }
            if (jsonData.status === 'ok') {
                switch (jsonData.response_type) {
                    case 'game_state':
                        window.Gaime.GameState = jsonData.game_state;
                        window.Gaime.username = this.username;
                        this.onDataReceived();
                        break;
                    case 'notification':
                        this.log(jsonData.notification);
                        break;
                    default:
                        console.warn('Received: ', event.data);
                        break;
                }
            } else {
                console.warn('Received: ', event.data);
            }
        });
    }

    quit() {
        this.disableQuitButton();
        this.enableTextareas();
        this.enableFindGameButton();
        this.doRender = false;
        this.websocket.close();
    }

    enableTextareas() {
        document.getElementById('initialisation-textarea').removeAttribute('disabled');
        document.getElementById('render-textarea').removeAttribute('disabled');
        document.getElementById('on-data-received-textarea').removeAttribute('disabled');
    }

    disableTextareas() {
        document.getElementById('initialisation-textarea').setAttribute('disabled', 'disabled');
        document.getElementById('render-textarea').setAttribute('disabled', 'disabled');
        document.getElementById('on-data-received-textarea').setAttribute('disabled', 'disabled');
    }

    enableFindGameButton() {
        document.getElementById('find-game-button').removeAttribute('disabled');
    }

    disableFindGameButton() {
        document.getElementById('find-game-button').setAttribute('disabled', 'disabled');
    }

    enableQuitButton() {
        document.getElementById('quit-button').removeAttribute('disabled');
    }

    disableQuitButton() {
        document.getElementById('quit-button').setAttribute('disabled', 'disabled');
    }

    compileScripts() {
        this.registerFunctionLog();
        this.registerFunctionSendGameCommand();
        const initialisation = new Function(this.initialisationScript);
        initialisation();
        this.onDataReceived = new Function(this.onDataReceivedScript);
        const render = new Function(this.renderScript);
        const renderLoop = () => {
            render();
            if (this.doRender === true) {
                window.requestAnimationFrame(renderLoop);
            }
        };
        this.doRender = true;
        window.requestAnimationFrame(renderLoop);
    }

    registerFunctionLog() {
        window.Gaime.log = (message) => {
            this.log(message);
        };
    }

    log(message) {
        console.log(message);
        this.logs += message + '\n';
    }

    error(message) {
        console.error(message);
        this.logs += '[ERROR] ' + message + '\n';
    }

    registerFunctionSendGameCommand() {
        window.Gaime.sendGameCommand = (game_command) => {
            const message = JSON.stringify({
                command: 'gamecommand',
                data: {
                    user : {
                        username: this.username,
                        jwt: this.jwt
                    },
                    game_command: game_command
                }
            });
            this.websocket.send(message);
        };
    }

    requestFindGame (username, jwt, game_string_id) {
        const message = JSON.stringify({
            command: 'findgame',
            data: {
                user : {
                    username: username,
                    jwt: jwt
                },
                game_string_id: game_string_id
            }
        });
        this.websocket.send(message);
    }

    getUser() {
        this.userService.getSelf()
            .subscribe((data: User) => {
                this.user = data;
            });
    }
}
