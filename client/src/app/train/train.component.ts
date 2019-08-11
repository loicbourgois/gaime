import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router'
import { GameService } from '../game/game.service';

@Component({
  selector: 'app-train',
  templateUrl: './train.component.html',
  styleUrls: ['./train.component.scss']
})
export class TrainComponent implements OnInit {

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
