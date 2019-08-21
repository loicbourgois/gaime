import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { HttpClientModule } from '@angular/common/http';
import { JwtModule } from '@auth0/angular-jwt';
import { FormsModule } from '@angular/forms';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { HomeComponent } from './home/home.component';
import { GameComponent } from './game/game.component';
import { TrainComponent } from './train/train.component';
import { SignupComponent } from './signup/signup.component';
import { UserComponent } from './user/user.component';
import { LoginComponent } from './login/login.component';
import { ProfileComponent } from './profile/profile.component';
import { PlayComponent } from './play/play.component';

@NgModule({
    declarations: [
        AppComponent,
        HomeComponent,
        GameComponent,
        TrainComponent,
        SignupComponent,
        UserComponent,
        LoginComponent,
        ProfileComponent,
        PlayComponent
    ],
    imports: [
        BrowserModule,
        HttpClientModule,
        AppRoutingModule,
        JwtModule.forRoot({
            config: {
                tokenGetter: () => {
                    const jwt_token = localStorage.getItem('jwt_token');
                    return jwt_token;
                },
                whitelistedDomains: ['localhost:8000'],
                blacklistedRoutes: []
            }
        }),
        FormsModule
    ],
    providers: [],
    bootstrap: [AppComponent]
})
export class AppModule { }
