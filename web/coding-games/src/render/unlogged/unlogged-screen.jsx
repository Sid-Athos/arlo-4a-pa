import {
    Alert,
    Box, Button, ButtonGroup,
    Card,
    CardActions,
    CardContent,
    Container,
    Fade, Modal,
    Stack,
    TextField,
    Typography
} from "@suid/material";
import {createMemo, createSignal, Match, Show, Switch} from "solid-js";
import dictionary from "../../utils/dictionary/dictionary";
import '../../index.css'

const UnloggedScreen = (
    {
        userSignIn,
        userSignInError,
        userSignInErrorMessage,
        setUserSignIn,
        setUserSignInError,
        userSignUp,
        userSignUpError,
        userSignUpErrorMessage,
        setUserSignUp,
        setUserSignUpError,
        submitSignUpFormOnPressEnter,
        submitSignInFormOnPressEnter,
        open,
        handleClose,
        signIn,
        signUp,
        userIsSignedIn,
        setUserIsSignedIn}) => {
    const [screen, setScreen] = createSignal('signIn');
    function swapScreen(newScreen){
        setScreen(newScreen);
        setScreen(newScreen);
        setUserSignUpError(false);
        setUserSignInError(false);
    }
    const  cardInfo = createMemo(() => {
        return dictionary["eng"].loginCard;
    })
    return (
        <Modal
            open={open()}
            onClose={handleClose}
            aria-labelledby="modal-modal-title"
            aria-describedby="modal-modal-description"
        >
            <Card
                className="signCard" sx={{m:3}}
            >
                <CardContent>
                    <Stack spacing={2}>

                        <Show when={userSignInError()} fallback={<></>}>
                            <Fade timeout={{ enter: 300, exit: 5000 }} in={userSignInError()} addEndListener={() => {
                                setTimeout(() => {
                                    setUserSignInError(false)
                                }, 5000);
                            }}
                            >
                                <Alert  severity="error">{userSignInErrorMessage()}</Alert>

                            </Fade>
                        </Show>
                        <Show when={userIsSignedIn()} fallback={<></>}>
                            <Fade timeout={{ enter: 300, exit: 5000 }} in={userIsSignedIn()} addEndListener={() => {
                                setTimeout(() => {
                                    setUserIsSignedIn(false);
                                }, 5000);
                            }}
                            >
                                <Alert className="darkThemeSuccessAlert" severity="success">Connexion successful</Alert>

                            </Fade>
                        </Show>
                        <Show when={userSignUpError()} fallback={<></>}>
                            <Fade timeout={{ enter: 300, exit: 5000 }} in={userSignInError()} addEndListener={() => {
                                setTimeout(() => {
                                    setUserSignInError(false)
                                }, 5000);
                            }}
                            >
                                <Alert severity="error">{userSignUpErrorMessage()}</Alert>
                            </Fade>
                        </Show>
                        <Switch>
                            <Match when={screen() === 'signIn'}>
                                <Stack component={"form"}
                                       sx={{
                                           width: '27ch',
                                       }}
                                       spacing={2}
                                       noValidate
                                       autoComplete="off"
                                >
                                <Typography className={"logScreenTypography"} gutterBottom>
                                    {cardInfo().signInButton}
                                </Typography>
                                    <Typography className={"logScreenTypography"} gutterBottom>
                                        Email / Username
                                    </Typography>
                                    <TextField
                                        required
                                        id="nickname"
                                        label="Required"
                                        type="text"
                                        variant="standard"
                                        onkeypress={async (k) => {
                                            await submitSignInFormOnPressEnter(k.key)
                                        }
                                        }
                                        inputProps={{ style: { color: "white", justifyContent:"center" } }}
                                        onchange={(e) =>{
                                            let user = userSignIn();
                                            user.nickname = e.target.value;
                                            setUserSignIn(user)
                                        } }
                                    />
                                    <Typography  className={"logScreenTypography"}gutterBottom>
                                        Password
                                    </Typography>
                                    <TextField
                                        id="standard-password-input"
                                        label="Password"
                                        type="password"
                                        autoComplete="current-password"
                                        inputProps={{ style: { color: "white" } }}
                                        onkeypress={async (k) => {
                                            await submitSignInFormOnPressEnter(k.key)
                                        }
                                        }
                                        variant="standard"
                                        v                     onChange={(e) =>{
                                        let user = userSignIn();
                                        user.password = e.target.value;
                                        setUserSignIn(user)
                                    } }
                                    />
                                    <Button size="large" onclick={signIn}>Login</Button>
                                </Stack>
                            </Match>
                            <Match when={screen() === 'signUp'}>
                                <Stack component={"form"}
                                       sx={{
                                           width: '27ch',
                                       }}
                                       spacing={2}
                                       noValidate
                                       autoComplete="off"
                                >
                                <Typography className={"logScreenTypography"} gutterBottom>
                                    {cardInfo().signUpButton}
                                </Typography>
                                    <Typography className={"logScreenTypography"} gutterBottom>
                                        Nickname
                                    </Typography>
                                    <TextField
                                        required
                                        id="username"
                                        label="Required"
                                        type="text"
                                        variant="standard"
                                        inputProps={{ style: { color: "white" } }}
                                        onkeypress={async (k) => {
                                            await submitSignUpFormOnPressEnter(k.key)
                                        }
                                        }
                                        onChange={(e) =>{
                                            let user = userSignUp();
                                            user.nickname = e.target.value;
                                            setUserSignUp(user)
                                        } }
                                    />
                                    <Typography className={"logScreenTypography"} gutterBottom>
                                        Email
                                    </Typography>
                                    <TextField
                                        required
                                        id="email"
                                        label="Required"
                                        type="email"
                                        variant="standard"
                                        inputProps={{ style: { color: "white" } }}
                                        onkeypress={async (k) => {
                                            await submitSignUpFormOnPressEnter(k.key)
                                        }
                                        }

                                        onChange={(e) =>{
                                            let user = userSignUp();
                                            user.email = e.target.value;
                                            setUserSignUp(user)
                                        } }
                                    />
                                    <Typography className={"logScreenTypography"} gutterBottom>
                                        Password
                                    </Typography>
                                    <TextField
                                        id="standard-password-input"
                                        label="Password"
                                        type="password"
                                        autoComplete="current-password"
                                        inputProps={{ style: { color: "white" } }}
                                        onkeypress={async (k) => {
                                            await submitSignUpFormOnPressEnter(k.key)
                                        }
                                        }
                                        variant="standard"
                                        v                     onChange={(e) =>{
                                        let user = userSignUp();
                                        user.password = e.target.value;
                                        setUserSignUp(user)
                                    } }
                                    />
                                    <Button size="large" onclick={signUp}>
                                        Register
                                    </Button>
                                </Stack>
                            </Match>
                        </Switch>
                    </Stack>
                </CardContent>
                <CardActions >
                    <Box sx={{paddingLeft: '20px'}}>
                    <ButtonGroup >
                        <Button size="large" onClick={() => swapScreen('signIn')}>
                            {cardInfo().signInButton}
                        </Button>
                        <Button size="large" onClick={() => swapScreen('signUp')} >
                            {cardInfo().signUpButton}
                        </Button>
                    </ButtonGroup>
                    </Box>
                </CardActions>
            </Card>
        </Modal>
    );
}

export default UnloggedScreen;