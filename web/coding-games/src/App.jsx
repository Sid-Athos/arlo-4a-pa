// @ts-ignore
import logo from './logo.svg';
// @ts-ignore
import styles from './App.module.css';
import {
    AppBar,
    Box,
    Button,
    IconButton,
    Toolbar,
    Typography
} from "@suid/material";
import MenuIcon from "@suid/icons-material/Menu";
import {createSignal } from "solid-js";
import axios from 'axios';
import { createStore } from "solid-js/store";
import UnloggedScreen from "./render/unlogged/unlogged-screen";

const instance = axios.create({
    baseURL: 'http://localhost:7590',
    headers: {
        'Content-Type': 'application/json',
        'api-key' : 'coding_games'
    },
});

const App = () => {
    const [user, setUser] = createStore({nickname:"", email:"", token: null});
    const [userSignIn, setUserSignIn] = createSignal({email : "", password: ""});
    const [userSignInError, setUserSignInError] = createSignal(false);
    const [userSignInErrorMessage, setUserSignInErrorMessage] = createSignal("");
    const [userSignUpErrorMessage, setUserSignUpErrorMessage] = createSignal("");
    const [userSignUp, setUserSignUp] = createSignal({email : "", password: "", nickname:""});
    const [userSignUpError, setUserSignUpError] = createSignal(false);
    const [open, setOpen] = createSignal(false);

    // @ts-ignore
    async function signIn() {
        if(userSignIn().email.length > 0 && userSignIn().password.length >= 3){
            try {
                const userLogged = await instance.post('/user/login', userSignIn());
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
        setAuthorizationHeader(userInfo.token)
        setUser(userInfo);
    }

    const setAuthorizationHeader = (token) => {
        instance.interceptors.request.use(conf => {
            conf.headers.setAuthorization(token)
            return conf
        })
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

    const handleOpen = () => setOpen(true);
    const handleClose = () => setOpen(false);

    return (
        <Box>
            <Box sx={{ flexGrow: 1 }}>
                <AppBar position="static" sx={{backgroundColor:'#282c34'}}>
                    <Toolbar>
                        <IconButton
                            size="large"
                            edge="start"
                            color="inherit"
                            aria-label="menu"
                            sx={{ mr: 2 }}
                        >
                            <MenuIcon />
                        </IconButton>
                        <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
                            News
                        </Typography>
                        <Button color="inherit" onClick={handleOpen}>Login</Button>
                    </Toolbar>
                </AppBar>
            </Box>
            <header class={styles.header}>
                <p>
                    Coding Games

                </p>
                <img src={logo} class={styles.logo} alt="logo" />
                <p>
                    Edit <code>src/App.tsx</code> and save to reload.
                </p>
            </header>
            <UnloggedScreen open={open} handleClose={handleClose()} userSignIn={userSignIn} userSignUp={userSignUp} userSignInError={userSignInError} userSignUpError={userSignUpError} signIn={signIn}
            userSignUpErrorMessage={userSignUpErrorMessage} submitSignInFormOnPressEnter={submitSignInFormOnPressEnter} submitSignUpFormOnPressEnter={submitSignUpFormOnPressEnter}
            userSignInErrorMessage={userSignInErrorMessage} setUserSignInError={setUserSignInError} setUserSignIn={setUserSignIn} setUserSignUp={setUserSignUp}
            setUserSignUpError={setUserSignUpError}/>
        </Box>
    );
};

export default App;
