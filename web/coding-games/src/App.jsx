
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
  CardContent,
  IconButton,
  Modal, TextField,
  Toolbar,
  Typography
} from "@suid/material";
import MenuIcon from "@suid/icons-material/Menu";
import {createResource, createSignal} from "solid-js";
import axios from 'axios';

const instance = axios.create({
    baseURL: 'http://localhost:7590',
    headers: {
        'Content-Type': 'application/json',
    },
});

const App = () => {


  const [open, setOpen] = createSignal(false);
  const handleOpen = () => setOpen(true);
  const handleClose = () => setOpen(false);
  const [userLogin, setUserLogin] = createSignal({email : "", password: ""});
    // @ts-ignore

  async function test() {
      console.log(userLogin())
      console.log(instance.getUri());
      const user = await instance.post('/user/login', userLogin());
      console.log(user);
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
              <Typography sx={{ fontSize: 22, textAlign: 'center'}} gutterBottom>
                Sign in
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
                    defaultValue="dsqdj"
                    onChange={(e) =>{
                      let user = userLogin();
                      user.email = e.target.value;
                      setUserLogin(user)
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
                    variant="standard"
                     onChange={(e) =>{
                      let user = userLogin();
                      user.password = e.target.value;
                      setUserLogin(user)
                    } }
                />
              </Box>
            </CardContent>
            <CardActions>
              <Button size="small" onClick={test}>Sign up</Button>
            </CardActions>
          </Card>
        </Modal>

      </Box>
  );
};

export default App;
