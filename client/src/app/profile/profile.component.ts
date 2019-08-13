import { Component, OnInit } from '@angular/core';
import { UserService } from '../user/user.service';
import { User } from '../user/user';

@Component({
    selector: 'app-profile',
    templateUrl: './profile.component.html',
    styleUrls: ['./profile.component.scss']
})
export class ProfileComponent implements OnInit {

    user: User;
    password_changed_succesfully: boolean = false;

    constructor(private userService: UserService) {
        this.user = new User();
    }

    ngOnInit() {
        this.getSelf();
    }

    getSelf() {
        this.userService.getSelf()
            .subscribe(data => {
                this.user.username = data['username'];
                this.user.email = data['email'];
            });
    }

    changePassword(password_1, password_2, new_password) {
        this.userService.changePassword(password_1, password_2, new_password)
            .subscribe((data: any) => {
                if (data.status === 'ok') {
                    this.userService.setJwtToken(data.jwt_token);
                    this.getSelf();
                    this.password_changed_succesfully = true;
                    setTimeout(() => {
                        this.password_changed_succesfully = false;
                    }, 2000);
                } else {
                    console.error(data.error);
                }
            });
    }

    changeEmail(new_email) {
        this.userService.changeEmail(new_email)
            .subscribe((data: any) => {
                if (data.status === 'ok') {
                    this.getSelf();
                } else {
                    console.error(data.error);
                }
            });
    }
}
