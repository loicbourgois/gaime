import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';

@Injectable({
  providedIn: 'root'
})
export class HomeService {

  helloUrl = 'http://localhost:8000/hello/';

  constructor(private http: HttpClient) { }

  getHello(name) {
    return this.http.get(this.helloUrl + name);
  }
}
