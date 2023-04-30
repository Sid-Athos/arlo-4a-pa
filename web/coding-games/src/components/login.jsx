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
    const [userSignIn, setUserSignIn] = createSignal({email : "", password: ""});
    const [userSignInError, setUserSignInError] = createSignal(false);
    const [userSignInErrorMessage, setUserSignInErrorMessage] = createSignal("");
    const [userSignUpErrorMessage, setUserSignUpErrorMessage] = createSignal("");
    const [userSignUp, setUserSignUp] = createSignal({email : "", password: "", nickname:""});
    const [userSignUpError, setUserSignUpError] = createSignal(false);

    // @ts-ignore
    async function signIn() {
        if(userSignIn().email.length > 0 && userSignIn().password.length >= 3){
            try {
                const userLogged = await UserService.signIn(userSignIn());
                if(userLogged.status === 200){
                    setClientData(userLogged)
                }
            } catch (error) {
                setUserSignInError(true)
                setUserSignInErrorMessage("An error occured while connecting")
            }
        }
    }

    const setClientData = (response) => {
        let userInfo = {...user}
        userInfo.email = userSignIn().email
        userInfo.token = "Bearer " + response.data.token
        AxiosInstance.updateAuthorizationHeader(userInfo.token)
        setUser(userInfo);
    }



    async function signUp() {
        if(userSignUp().email.length > 0 && userSignUp().password.length >= 3 && userSignUp().nickname.length >= 5){
            try {
                const userCreated = await instance.post('/user/create', userSignUp());
                if(userCreated.status === 200){
                    console.log(userCreated.data)
                }
            } catch (error) {
                setUserSignUpError(!userSignInError())
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
            <header class={styles.header}>
                <p>
                    Coding Games
                </p>
                <img src={logo} class={styles.logo} alt="logo" />
            </header>
            <UnloggedScreen open={open} handleClose={handleClose} userSignIn={userSignIn} userSignUp={userSignUp} userSignInError={userSignInError} userSignUpError={userSignUpError} signIn={signIn}
            userSignUpErrorMessage={userSignUpErrorMessage} submitSignInFormOnPressEnter={submitSignInFormOnPressEnter} submitSignUpFormOnPressEnter={submitSignUpFormOnPressEnter}
            userSignInErrorMessage={userSignInErrorMessage} setUserSignInError={setUserSignInError} setUserSignIn={setUserSignIn} setUserSignUp={setUserSignUp}
            setUserSignUpError={setUserSignUpError}/>
        </Box>
    );
};

