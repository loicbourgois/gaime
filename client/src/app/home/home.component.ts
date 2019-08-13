import { Component, OnInit } from '@angular/core';
import { HomeService } from './home.service';
import { GameService } from '../game/game.service';
import { Game } from '../game/game';

@Component({
    selector: 'app-home',
    templateUrl: './home.component.html',
    styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {

    games: Game[] = [];
    gamee: Game;

    constructor(
        private homeService: HomeService,
        private gameService: GameService
    ) { }

    ngOnInit() {
        this.getGames();
    }

    getGames() {
        this.gameService.getGames()
            .subscribe((data: any) => {
                this.games = data;
            });
    }
}
