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
        const game_websocket_url: string = this.game.websocket_url;
        const username: String = this.user.username;
        const jwt: string = this.userService.getJwt();
        const game_string_id = this.game.string_id;
        if (!game_websocket_url) {
            console.error('game_websocket_url', game_websocket_url);
            return;
        }
        if (!username) {
            console.error('username', username);
            return;
        }
        if (!jwt) {
            console.error('jwt', jwt);
            return;
        }
        if (!game_string_id) {
            console.error('game_string_id', game_string_id);
            return;
        }
        this.websocket = new WebSocket(game_websocket_url);
        this.websocket.addEventListener('open', event => {
            this.requestFindGame(username, jwt, game_string_id);
        });
        this.websocket.addEventListener('message', function (event) {
            console.log('Received: ', event.data);
        });
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
