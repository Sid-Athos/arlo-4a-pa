import {
    Alert,
    Box, Button,
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
        signIn}) => {
    const [screen, setScreen] = createSignal('signIn');
    function swapScreen(){
        if(screen() === "signIn"){
            setScreen('signUp');
        } else {
            setScreen('signIn');
        }
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
                    <Stack spacing={2}>

                        <Show when={userSignInError()} fallback={<></>}>
                            <Fade timeout={{ enter: 300, exit: 5000 }} in={userSignInError()} addEndListener={() => {
                                setTimeout(() => {
                                    setUserSignInError(false)
                                }, 5000);
                            }}
                            >
                                <Alert sx={{backgroundColor:"rgb(22.899999999999995, 11.499999999999998, 11.499999999999998)", color:"rgb(244.6, 199, 199)"}} severity="error">{userSignInErrorMessage()}</Alert>

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
                                        onkeypress={async (k) => {
                                            await submitSignInFormOnPressEnter(k.key)
                                        }
                                        }
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
                                </Box>
                            </Match>
                            <Match when={screen() === 'signUp'}>
                                <Typography sx={{ fontSize: 22, textAlign: 'center'}} gutterBottom>
                                    {cardInfo().signInButton}
                                </Typography>
                                <Box component={"form"}>
                                    <Typography sx={{ fontSize: 18, textAlign: 'center'}} gutterBottom>
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
                                        sx={{  color: '#ffffff'  }}
                                    />
                                    <Typography sx={{ fontSize: 18, textAlign: 'center'}} gutterBottom>
                                        Email
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
                                </Box>
                            </Match>
                        </Switch>
                    </Stack>
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
    );
}

export default UnloggedScreen;