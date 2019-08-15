import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router'
import { Game } from '../game/game';
import { GameService } from '../game/game.service';
import { User } from '../user/user';
import { UserService } from '../user/user.service';


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
        this.websocket.addEventListener('open', event => {
            this.disableTextareas();
            this.disableFindGameButton();
            this.enableQuitButton();
            this.compileScripts();
            this.requestFindGame(this.username, this.jwt, game_string_id);
        });
        this.websocket.addEventListener('message', (event) => {
            let jsonData = {};
            try {
                jsonData = JSON.parse(event.data);
            } catch (error) {
                // console.warn(error);
            }
            if (jsonData.status === 'ok') {
                if (jsonData.response_type === 'game_state') {
                    window.Gaime.GameState = jsonData.game_state;
                    window.Gaime.GameState.username = this.username;
                    this.onDataReceived();
                } else {
                    console.warn('Received: ', event.data);
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
        if (!window.Gaime) {
            window.Gaime = {};
        } else {
            window.Gaime.GameState = null;
        }
        this.registerFunctionLog();
        this.registerFunctionSendGameCommand();
        const initialisationScript = document.getElementById('initialisation-textarea').value;
        const initialisation = new Function(initialisationScript);
        initialisation();
        const onDataReceivedScript = document.getElementById('on-data-received-textarea').value;
        this.onDataReceived = new Function(onDataReceivedScript);
        const renderScript = document.getElementById('render-textarea').value;
        const render = new Function(renderScript);
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
            console.log(message);
            document.getElementById('log-textarea').value += message + '\n';
        };
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
