import { Component, OnInit } from '@angular/core';
import { UserService } from '../user/user.service';

@Component({
    selector: 'app-signup',
    templateUrl: './signup.component.html',
    styleUrls: ['./signup.component.scss']
})
export class SignupComponent implements OnInit {

    constructor(
        private userService: UserService
    ) { }

    ngOnInit() {
    }

    signup(username, password, email) {
        this.userService.signup(username, password, email)
            .subscribe((data: any) => {
                if (data.status === 'ok') {
                    this.userService.login(username, password);
                } else {
                    console.error(data);
                }
            });
    }
}
