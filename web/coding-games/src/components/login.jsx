// @ts-ignore
import logo from '../logo.svg';
// @ts-ignore
import styles from '../App.module.css';
import {
    Box,
} from "@suid/material";
import {createSignal } from "solid-js";
import { createStore } from "solid-js/store";
import UnLoggedScreen from "../render/unlogged/unlogged-screen";
import {ApiInstance} from "../utils/services/api-instance";
import {GameManagerInstance} from "../utils/services/game-manager-instance";
import {UserService} from "../utils/services/user-service";
import {useNavigate} from "@solidjs/router";
import {UserStore} from "../utils/user-store";
import useSockets from "../hooks/useSockets";

export default function LoginComponent ({open,setOpen})  {
    const [user, setUser] = createStore({nickname:"", email:"", token: null});
    const [userSignIn, setUserSignIn] = createSignal({nickname : "", password: ""});
    const [userSignInError, setUserSignInError] = createSignal(false);
    const [userSignInErrorMessage, setUserSignInErrorMessage] = createSignal("");
    const [userSignUpErrorMessage, setUserSignUpErrorMessage] = createSignal("");
    const [userSignUp, setUserSignUp] = createSignal({email : "", password: "", nickname:""});
    const [userSignUpError, setUserSignUpError] = createSignal(false);
    const [userIsSignedIn, setUserIsSignedIn] = createSignal(false);
    const navigate = useNavigate();
    // @ts-ignore
    async function signIn() {
        if(userSignIn().nickname.length >= 5 && userSignIn().password.length >= 8){
            try {
                let res = await UserService.signIn(userSignIn());
                if(res.status === 200){
                    setClientData(res);
                    setUserSignInError(false)
                    setUserIsSignedIn(true)
                    navigate("/lobby")
                    handleClose();
                }
            } catch (error) {
                setUserSignInError(true)
                setUserSignInErrorMessage("An error occured while connecting")
            }
        } else {
            alert("teubé")
        }
    }

    const setClientData = (response) => {
        let userInfo = {...user}
        userInfo.nickname = userSignIn().nickname
        userInfo.token = "Bearer " + response.data.token
        ApiInstance.updateAuthorizationHeader(userInfo.token );
        GameManagerInstance.updateAuthorizationHeader(userInfo.token);
        UserStore.save({token: response.data.token, username: userInfo.nickname, mail: userInfo.email})
        sessionStorage.setItem("token", response.data.token)
        sessionStorage.setItem("username", userInfo.nickname)
    }



    async function signUp() {
        if(userSignUp().email.length > 0 && userSignUp().password.length >= 3 && userSignUp().nickname.length >= 5){
            try {
                const userCreated = await UserService.signUp(userSignUp());
                if(userCreated.status === 200){
                    let res = await UserService.signIn({nickname: userSignUp().nickname, password: userSignUp().nickname})
                    setClientData(res);
                    setUserSignInError(false)
                    setUserIsSignedIn(true)
                    navigate("/lobby")
                    handleClose();
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
            <UnLoggedScreen open={open} handleClose={handleClose} userSignIn={userSignIn} userSignUp={userSignUp} userSignInError={userSignInError} userSignUpError={userSignUpError} signIn={signIn}
                            userSignUpErrorMessage={userSignUpErrorMessage} submitSignInFormOnPressEnter={submitSignInFormOnPressEnter} submitSignUpFormOnPressEnter={submitSignUpFormOnPressEnter}
                            userSignInErrorMessage={userSignInErrorMessage} setUserSignInError={setUserSignInError} setUserSignIn={setUserSignIn} setUserSignUp={setUserSignUp}
                            setUserSignUpError={setUserSignUpError} signUp={signUp} userIsSignedIn={userIsSignedIn} setUserIsSignedIn={setUserIsSignedIn}></UnLoggedScreen>
        </Box>
    );
};
