
// @ts-ignore
import logo from './logo.svg';
// @ts-ignore
import styles from './App.module.css';
import {
    AppBar,
    Box,
    Button,
    Card,
    CardActions,
    CardContent, Container,
    IconButton,
    Modal, Stack, TextField,
    Toolbar,
    Typography
} from "@suid/material";
import MenuIcon from "@suid/icons-material/Menu";
import {createMemo, createResource, createSignal, Match, Show, Switch} from "solid-js";
import axios from 'axios';
import dictionary from "./utils/dictionary/dictionary";
import { Alert } from "@suid/material"

const instance = axios.create({
    baseURL: 'http://localhost:7590',
    headers: {
        'Content-Type': 'application/json',
        'api-key' : 'coding_games'
    },
});

const App = () => {


    const [open, setOpen] = createSignal(false);
    const [screen, setScreen] = createSignal('signIn');

    const handleOpen = () => setOpen(true);
    const handleClose = () => setOpen(false);

    const  cardInfo = createMemo(() => {
        return dictionary["eng"].loginCard;
    })
    const [userSignIn, setUserSignIn] = createSignal({email : "", password: ""});
    const [userSignInError, setUserSignInError] = createSignal(false);
    const [userSignInErrorMessage, setUserSignInErrorMessage] = createSignal("");
    const [userSignUpErrorMessage, setUserSignUpErrorMessage] = createSignal("");

    const [userSignUp, setUserSignUp] = createSignal({email : "", password: "", nickname:""});
    const [userSignUpError, setUserSignUpError] = createSignal(false);

    // @ts-ignore

    async function signIn() {
        console.log(userSignIn())
        if(userSignIn().email.length > 0 && userSignIn().password.length >= 3){
            const userLogged = await instance.post('/user/login', userSignIn());
            if(userLogged.status === 200){
                console.log(userLogged.data)
                return;
            }
            setUserSignInError(!userSignInError())
            setUserSignInErrorMessage("An error occured while connecting")
        }
    }

    async function signup() {
        if(userSignUp().email.length > 0 && userSignUp().password.length >= 3 && userSignUp().nickname.length >= 5){
            const userCreated = await instance.post('/user/create', userSignUp());
            if(userCreated.status === 200){
                console.log(userCreated.data)
            }
            setUserSignUpError(!userSignInError())
            setUserSignUpErrorMessage("An error occured while creating your account")
        }
    }

    function swapScreen(){
        if(screen() === "signIn"){
            setScreen('signUp');
        } else {
            setScreen('signIn');
        }
        setUserSignUpError(false);
        setUserSignInError(false);
    }

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
            <Switch>
                <Match when={userSignInError()}>
                    <Alert severity="error">{userSignInErrorMessage()}</Alert>
                </Match>
                <Match when={userSignUpError()}>
                    <Alert severity="error">{userSignUpErrorMessage()}</Alert>
                </Match>
            </Switch>
            <header class={styles.header}>
                <p>
                    Coding Games

                </p>
                <img src={logo} class={styles.logo} alt="logo" />
                <p>
                    Edit <code>src/App.tsx</code> and save to reload.
                </p>
            </header>
            <Modal
                open={open()}
                onClose={handleClose}
                aria-labelledby="modal-modal-title"
                aria-describedby="modal-modal-description"
            >
                <Card
                    sx={{
                        position: "absolute",
                        top: "30%",
                        left: "53%",
                        transform: "translate(-50%, -50%)",
                        width: 300,
                        backgroundColor: '#282c34',
                        border: "2px solid #000",
                        boxShadow: "24px",
                        color: 'white',
                        p: 4,
                    }}
                >
                    <CardContent>
                        <Switch>
                            <Match when={screen() === 'signIn'}>
                                <Typography sx={{ fontSize: 22, textAlign: 'center'}} gutterBottom>
                                    {cardInfo().signInButton}
                                </Typography>
                                <Box component={"form"}>
                                    <Typography sx={{ fontSize: 18, textAlign: 'center'}} gutterBottom>
                                        Email / Username
                                    </Typography>
                                    <TextField
                                        required
                                        id="email"
                                        label="Required"
                                        type="email"
                                        variant="standard"
                                        inputProps={{ style: { color: "white" } }}
                                        onChange={(e) =>{
                                            let user = userSignIn();
                                            user.email = e.target.value;
                                            setUserSignIn(user)
                                        } }
                                        sx={{  color: '#ffffff'  }}
                                    />
                                    <Typography sx={{ fontSize: 22, textAlign: 'center'}} gutterBottom>
                                        Password
                                    </Typography>
                                    <TextField
                                        id="standard-password-input"
                                        label="Password"
                                        type="password"
                                        autoComplete="current-password"
                                        inputProps={{ style: { color: "white" } }}

                                        variant="standard"
                                        v                     onChange={(e) =>{
                                        let user = userSignIn();
                                        user.password = e.target.value;
                                        setUserSignIn(user)
                                    } }
                                    />
                                </Box>
                            </Match>
                            <Match when={screen() === 'signUp'}>
                                <Typography sx={{ fontSize: 22, textAlign: 'center'}} gutterBottom>
                                    {cardInfo().signInButton}
                                </Typography>
                                <Box component={"form"}>
                                    <Typography sx={{ fontSize: 18, textAlign: 'center'}} gutterBottom>
                                        Email / Username
                                    </Typography>
                                    <TextField
                                        required
                                        id="email"
                                        label="Required"
                                        type="email"
                                        variant="standard"
                                        inputProps={{ style: { color: "white" } }}
                                        onChange={(e) =>{
                                            let user = userSignUp();
                                            user.email = e.target.value;
                                            setUserSignUp(user)
                                        } }
                                        sx={{  color: '#ffffff'  }}
                                    />
                                    <Typography sx={{ fontSize: 22, textAlign: 'center'}} gutterBottom>
                                        Password
                                    </Typography>
                                    <TextField
                                        id="standard-password-input"
                                        label="Password"
                                        type="password"
                                        autoComplete="current-password"
                                        inputProps={{ style: { color: "white" } }}

                                        variant="standard"
                                        v                     onChange={(e) =>{
                                        let user = userSignUp();
                                        user.password = e.target.value;
                                        setUserSignUp(user)
                                    } }
                                    />
                                </Box>
                            </Match>
                        </Switch>
                    </CardContent>
                    <CardActions>
                        <Container>
                            <Button size="small" onClick={signIn} sx={{left:'40px'}}>
                                {cardInfo().signInButton}
                            </Button>
                        </Container>
                        <Container>
                            <Button size="small" onClick={swapScreen} >
                                {cardInfo().signUpButton}
                            </Button>
                        </Container>
                    </CardActions>
                </Card>
            </Modal>

        </Box>
    );
};

export default App;
