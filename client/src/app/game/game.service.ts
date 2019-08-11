import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';

@Injectable({
  providedIn: 'root'
})
export class GameService {

  baseUrl = 'http://localhost:8000';

  constructor(private http: HttpClient) { }

  getGames() {
    return this.http.get(`${this.baseUrl}/games`);
  }

  getGame(gameId) {
    return this.http.get(`${this.baseUrl}/game/${gameId}`);
  }
}
