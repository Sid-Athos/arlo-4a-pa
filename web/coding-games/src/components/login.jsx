// @ts-ignore
import logo from '../logo.svg';
// @ts-ignore
import styles from '../App.module.css';
import {
    Box,
} from "@suid/material";
import {createSignal } from "solid-js";
import { createStore } from "solid-js/store";
import UnloggedScreen from "../render/unlogged/unlogged-screen";

import {AxiosInstance} from "../utils/services/axios-instance";
import {UserService} from "../utils/services/user-service";

export default function LoginComponent ({open, setOpen})  {
    const [user, setUser] = createStore({nickname:"", email:"", token: null});
    const [userSignIn, setUserSignIn] = createSignal({nickname : "", password: ""});
    const [userSignInError, setUserSignInError] = createSignal(false);
    const [userSignInErrorMessage, setUserSignInErrorMessage] = createSignal("");
    const [userSignUpErrorMessage, setUserSignUpErrorMessage] = createSignal("");
    const [userSignUp, setUserSignUp] = createSignal({email : "", password: "", nickname:""});
    const [userSignUpError, setUserSignUpError] = createSignal(false);
    const [userIsSignedIn, setUserIsSignedIn] = createSignal(false);

    // @ts-ignore
    async function signIn() {
        if(userSignIn().nickname.length > 5 && userSignIn().password.length >= 8){
            try {
                let res = await UserService.signIn(userSignIn());
                if(res.status === 200){
                    setClientData(res);
                    setUserSignInError(false)
                    setUserIsSignedIn(true)
                }
            } catch (error) {
                setUserSignInError(true)
                setUserSignInErrorMessage("An error occured while connecting")
            }
        }
    }

    const setClientData = (response) => {
        let userInfo = {...user}
        userInfo.nickname = userSignIn().nickname
        userInfo.token = "Bearer " + response.data.token
        AxiosInstance.updateAuthorizationHeader(userInfo.token)
        setUser(userInfo);
    }



    async function signUp() {
        if(userSignUp().email.length > 0 && userSignUp().password.length >= 3 && userSignUp().nickname.length >= 5){
            try {
                const userCreated = await UserService.signUp(userSignUp());
                if(userCreated.status === 200){
                    setClientData(userCreated);
                    setUserSignInError(false)
                    setUserIsSignedIn(true)
                }
            } catch (error) {
                setUserSignUpError(!userSignUpError())
                setUserSignUpErrorMessage("An error occured while creating your account")
            }
        }
    }


    async function submitSignInFormOnPressEnter(key){
        if (key === 'Enter') {
            await signIn()
        }
    }

    async function submitSignUpFormOnPressEnter(key){
        if (key === 'Enter') {
            await signUp()
        }
    }

    const handleClose = () => setOpen(false);

    return (
        <Box>
            <header className={styles.header}>
                <p>
                    Coding Games
                </p>
                <img src={logo} className={styles.logo} alt="logo" />
            </header>
            <UnloggedScreen open={open} handleClose={handleClose} userSignIn={userSignIn} userSignUp={userSignUp} userSignInError={userSignInError} userSignUpError={userSignUpError} signIn={signIn}
            userSignUpErrorMessage={userSignUpErrorMessage} submitSignInFormOnPressEnter={submitSignInFormOnPressEnter} submitSignUpFormOnPressEnter={submitSignUpFormOnPressEnter}
            userSignInErrorMessage={userSignInErrorMessage} setUserSignInError={setUserSignInError} setUserSignIn={setUserSignIn} setUserSignUp={setUserSignUp}
            setUserSignUpError={setUserSignUpError} signUp={signUp} userIsSignedIn={userIsSignedIn} setUserIsSignedIn={setUserIsSignedIn}></UnloggedScreen>
        </Box>
    );
};

