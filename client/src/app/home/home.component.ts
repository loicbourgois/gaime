import { Component, OnInit } from '@angular/core';
import { HomeService } from './home.service';
import { GameService } from '../game/game.service';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {

  games;

  constructor(
    private homeService: HomeService,
    private gameService: GameService
  ) { }

  ngOnInit() {
    this.getGames();
  }

  getGames() {
    this.gameService.getGames()
      .subscribe(data => {
        this.games = data.games;
      });
  }
}
