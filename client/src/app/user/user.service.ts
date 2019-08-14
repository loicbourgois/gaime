import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
const bcrypt = require('bcryptjs');
import { Router } from '@angular/router';


const httpOptions = {
    headers: new HttpHeaders({
        'Content-Type':  'application/json'
    })
};


@Injectable({
    providedIn: 'root'
})
export class UserService {

    baseUrl = 'http://localhost:8000';

    constructor (
        private http: HttpClient,
        private router: Router
    ) { }

    //
    // Uses the Authorization header to get the corresponding User infos
    //
    getSelf() {
        return this.http.get(`${this.baseUrl}/self`);
    }

    signup(username, password, email) {
        return this.http.post(
            `${this.baseUrl}/signup`,
            {
                username: username,
                password: password,
                email: email
            },
            httpOptions
        );
    }

    login(username, password) {
        this.http.post(
            `${this.baseUrl}/login`,
            {
                username: username,
                password: password
            },
            httpOptions
        ).subscribe((data: any) => {
            if (data.status === 'ok') {
                this.setJwtToken(data['jwt_token']);
                this.router.navigate(['/profile']);
            } else {
                console.error(data.error);
            }
        });
    }

    changePassword(password_1, password_2, new_password) {
        return this.http.post(
            `${this.baseUrl}/changepassword`,
            {
                password_1: password_1,
                password_2: password_2,
                new_password: new_password
            },
            httpOptions
        );
    }

    changeEmail(new_email) {
        return this.http.post(
            `${this.baseUrl}/changeemail`,
            {
                new_email: new_email
            },
            httpOptions
        );
    }

    setJwtToken(jwt_token) {
        localStorage.setItem('jwt_token', jwt_token);
    }

    getJwt() {
        return localStorage.getItem('jwt_token');
    }
}
