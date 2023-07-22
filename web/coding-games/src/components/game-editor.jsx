import { CodeMirror } from "@solid-codemirror/codemirror";
import { basicSetup } from "codemirror";
import { java } from "@codemirror/lang-java";
import {python} from "@codemirror/lang-python";
import {
    Box, Button,
    Container,
    Divider, Drawer, FormControl, FormHelperText,
    Grid, InputLabel,
    List,
    ListItem,
    ListItemButton,
    ListItemIcon,
    ListItemText, MenuItem, Select, TextField, Typography
} from "@suid/material";
import { oneDark } from "@codemirror/theme-one-dark";
import {createResource, createSignal, lazy, onMount, Show, Suspense} from "solid-js";
import {createMutable} from "solid-js/store";
import {DeleteOutlineOutlined, SaveAltOutlined, VideogameAssetOutlined} from "@suid/icons-material";
import {GamesService} from "../utils/services/game-manager-service";


export default function Editor(){
    const [codes, setCodes] = createSignal( [])
    onMount(async () => {
      let res = await GamesService.findMyGames();
      console.log(res)
        setCodes(res.data)
    });
    const [drawerState,setDrawerState] = createSignal(false);
    const [minPlayers,setMinPlayers] = createSignal(2);
    const [maxPlayers,setMaxPlayers] = createSignal(2);
    const [description,setDescription] = createSignal("");
    const [language, setLanguage] = createSignal(python())
    const [languageString, setLanguageString] = createSignal("python")

    const toggleDrawer =
        (anchor, open) => (event) => {

            if (event.type === "keydown") {
                const keyboardEvent = event;
                if (keyboardEvent.key === "Tab" || keyboardEvent.key === "Shift")
                    return;
            }
            setDrawerState(open);

        };

    const list = (anchor) => (

        <Box
            role="presentation"
            onClick={toggleDrawer(anchor, false)}
            onKeyDown={toggleDrawer(anchor, false)}
            sx={{}}
        >
            <List>
                {["Save", "Delete"].map((text, index) => (
                    <ListItem disablePadding>
                        <ListItemButton onclick={() => clickDrawerItem(text)}>
                            <ListItemIcon sx={{color:'white'}}>
                                {text === "Save"?<SaveAltOutlined></SaveAltOutlined>: <DeleteOutlineOutlined></DeleteOutlineOutlined>}


                            </ListItemIcon>
                            <ListItemText primary={text} />
                        </ListItemButton>
                    </ListItem>
                ))}
            </List>
            <Divider />
            <Typography sx={{textAlign:'center'}}>Game list</Typography>
            <List>
                {["Morpion", "Puissance 4"].map((text, index) => (
                    <ListItem disablePadding>
                        <ListItemButton onclick={() => clickDrawerGameItem(text)}>
                            <ListItemIcon sx={{color:'white'}}>
                                <VideogameAssetOutlined></VideogameAssetOutlined>
                            </ListItemIcon>
                            <ListItemText primary={text} />
                        </ListItemButton>
                    </ListItem>
                ))}
            </List>
        </Box>
    );
    const clickDrawerItem = (e) => {
        //console.log(e)
    }
    const clickDrawerGameItem = (e) => {
       // console.log(e)
    }
    const swapLanguage = (language) => {
        switch (language) {
            case 'python':
                setLanguage(python())
                setLanguageString("python")
                break;
            case 'java':
                setLanguage(java())
                setLanguageString("java")
                break;
            default:
                alert("ton langage n'existe pas")
        }
    }
    const checkPlayersAmount = (e) => {
        if(e.target.value < minPlayers()){
           return alert("tu te fous de ma gueule")
        }
        setMaxPlayers(e.target.value)
    }
    const saveOnCtrlS = (e) => {
        if(e.ctrlKey && e.key === "s"){
            e.preventDefault()
        }
    }
    return (
        <>

                    <Drawer
                        PaperProps={{
                            sx: {
                                backgroundColor: "s282c34",
                                color:"white"
                            }
                        }}
                    anchor={"right"}
                    open={drawerState()}
                    sx={{ zIndex: 9999 }}
                    onClose={toggleDrawer("right", false)}
                    >
                {list("right")}
                    </Drawer>
            <Container maxWidth="sm" sx={{paddingTop: '50px'}}>
            <Box sx={{ flexGrow: 1 }}>
                <Grid container spacing={2}>
                    <Grid item xs={6} md={8}>
                        <Show when={codes().length > 0}>
                            <CodeMirror value={codes()[0].code} extensions={[basicSetup, java()]} theme={oneDark} onKeyDown={saveOnCtrlS} />
                        </Show>
                    </Grid>
                    <Grid item xs={6} md={4}>
                    <Button onClick={toggleDrawer("right", true)} >{"Contextual menu"}</Button>
                        <FormControl
                            sx={{
                                m: 1,
                                minWidth: 120,
                            }}
                        >
                            <InputLabel id="demo-simple-select-disabled-label">Program Language</InputLabel>
                            <Select
                                labelId="demo-simple-select-disabled-label"
                                id="demo-simple-select-disabled"
                                value={languageString()}
                                onChange={(e) => swapLanguage(e.target.value)}
                                label="Language"
                            >
                                <MenuItem value={'python'}>python</MenuItem>
                                <MenuItem value={'java'}>java</MenuItem>
                            </Select>
                        </FormControl>
                        <FormControl
                            sx={{
                                m: 1,
                                minWidth: 120,
                            }}
                        >
                            <InputLabel id="demo-simple-select-disabled-label">Minimum players</InputLabel>
                            <Select
                                labelId="demo-simple-select-disabled-label"
                                id="demo-simple-select-disabled"
                                value={minPlayers()}
                                onChange={(e) => setMinPlayers(e.target.value)}
                                label="Minimum players"
                            >
                                <MenuItem value={2}>2</MenuItem>
                                <MenuItem value={3}>3</MenuItem>
                                <MenuItem value={4}>4</MenuItem>
                                <MenuItem value={5}>5</MenuItem>
                                <MenuItem value={6}>6</MenuItem>
                                <MenuItem value={7}>7</MenuItem>
                                <MenuItem value={8}>8</MenuItem>
                            </Select>
                        </FormControl>
                        <FormControl sx={{
                                m: 1,
                                minWidth: 120,
                            }}>
                            <InputLabel id="demo-simple-select-disabled-label">Maximum players</InputLabel>
                            <Select
                                labelId="demo-simple-select-disabled-label"
                                id="demo-simple-select-disabled"
                                value={maxPlayers()}
                                label="Minimum players"
                                onChange={checkPlayersAmount}
                            >
                                <MenuItem value={2}>2</MenuItem>
                                <MenuItem value={3}>3</MenuItem>
                                <MenuItem value={4}>4</MenuItem>
                                <MenuItem value={5}>5</MenuItem>
                                <MenuItem value={6}>6</MenuItem>
                                <MenuItem value={7}>7</MenuItem>
                                <MenuItem value={8}>8</MenuItem>
                            </Select>
                        </FormControl>
                        <FormControl>
                            <TextField
                                id="filled-multiline-flexible"
                                label="Game Description"
                                multiline
                                maxRows={4}
                                value={description()}
                                variant="filled"
                                onChange={(e) => setDescription(e.target.value)}
                            />
                        </FormControl>
                    </Grid>
                </Grid>
            </Box>
            </Container>

        </>
    )
}