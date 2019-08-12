import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { GameService } from './game.service';

@Component({
  selector: 'app-game',
  templateUrl: './game.component.html',
  styleUrls: ['./game.component.scss']
})
export class GameComponent implements OnInit {

  game = {};

  constructor(
    private route: ActivatedRoute,
    private gameService: GameService
  ) {
  }

  ngOnInit() {
    this.getGame();
  }

  getGame(): void {
    const gameId = this.route.snapshot.paramMap.get('gameId');
    this.gameService.getGame(gameId)
      .subscribe(data => {
        this.game = data;
      });
  }

}
