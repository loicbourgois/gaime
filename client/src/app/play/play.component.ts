import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router'
import { GameService } from '../game/game.service';


@Component({
    selector: 'app-play',
    templateUrl: './play.component.html',
    styleUrls: ['./play.component.scss']
})
export class PlayComponent implements OnInit {

    game = {};

    constructor(
        private route: ActivatedRoute,
        private gameService: GameService
    ) {
    }

    ngOnInit() {
        this.getGame();
        // Create WebSocket connection.
        const socket = new WebSocket('ws://0.0.0.0:8080');

        // Connection opened
        socket.addEventListener('open', function (event) {
                socket.send('loginjohn');
        });

        // Listen for messages
        socket.addEventListener('message', function (event) {
                console.log('Received: ', event.data);
        });
    }

    getGame(): void {
        const gameId = this.route.snapshot.paramMap.get('gameId');
        this.gameService.getGame(gameId)
            .subscribe(data => {
                this.game = data;
            });
    }

}
