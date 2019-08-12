import { Component, OnInit } from '@angular/core';
import { UserService } from '../user/user.service';

@Component({
      selector: 'app-login',
      templateUrl: './login.component.html',
      styleUrls: ['./login.component.scss']
})


export class LoginComponent implements OnInit {

    constructor(private userService: UserService) { }

    ngOnInit() {
    }

    login(username, password) {
        this.userService.login(username, password);
    }
}
