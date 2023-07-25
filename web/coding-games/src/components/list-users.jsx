import {createMemo, createSignal, For, onMount, Show} from "solid-js";
import {UserService} from "../utils/services/user-service";
import {
    Box, Button, createTheme,
    IconButton,
    List,
    ListItem,
    ListItemButton,
    ListItemIcon,
    ListItemText,
    Paper, Table, TableBody, TableCell, TableContainer,
    TableHead, TableRow, ThemeProvider
} from "@suid/material";
import SearchComponent from "./search-bar.jsx"
import createSvgIcon from "@suid/material/utils/createSvgIcon";
import {UserStore} from "../utils/user-store";
import {Add} from "@suid/icons-material";
import {useNavigate} from "@solidjs/router";


export default function ListUsers() {
    const nav = useNavigate();
    console.log(UserStore.get().pseudo)
    const PlusIcon = createSvgIcon(
        // credit: plus icon from https://heroicons.com/
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            strokeWidth={1.5}
            stroke="currentColor"
        >
            <path strokeLinecap="round" strokeLinejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
        </svg>,
        'Plus',
    );
    const [userList, setUserList] = createSignal([])

    const users = createMemo(()=> {
        console.log(userList())
        console.log(UserStore.get().username)
        console.log(userList().filter(user => user.pseudo !== UserStore.get().username))
        if(UserStore.get().username){
        return userList().filter(user => user.pseudo !== UserStore.get().username)

        } else if (sessionStorage.getItem("username")){
             return userList().filter(user => user.pseudo !== sessionStorage.getItem("username"))

        } else {
            nav("/")
        }
    })
    console.log(users())

    const addFriend = async (recipientId) => {

        await UserService.addFriend({recipient_id: recipientId})
    }

    function renderManagementOptions(userId) {
        return (
            <>
                <IconButton aria-label="delete" onClick={() => addFriend(userId)}>
                    <Add />
                </IconButton>

            </>);
    }
    const darkTheme = createTheme({
        palette: {
            mode: 'dark',
        },
    });
    return (
        <>
            <Box sx={{flexGrow: 1, m: 8}}>
                <SearchComponent setUserList={setUserList}></SearchComponent>
                <Show when={userList().length > 0}>
                    <ThemeProvider theme={darkTheme}>
                    <TableContainer component={Paper}>
                        <Table sx={{maxWidth: 650}} aria-label="simple table">
                            <TableHead>
                                <TableRow>
                                    <TableCell>Username</TableCell>
                                    <TableCell align="right">Experience</TableCell>
                                    <TableCell align="right">Level</TableCell>
                                    <TableCell align="right">Actions</TableCell>
                                </TableRow>
                            </TableHead>
                            <TableBody>
                        { users().map(user => {
                            return (    <><TableRow>
            <TableCell>
                {user.pseudo}
            </TableCell>
                                    <TableCell align="right">
                                        {user.experience}
                                    </TableCell>

                                    <TableCell align="right">
                                        {user.level}
                                    </TableCell>
                                    <TableCell align="right">
                                        {renderManagementOptions}
                                    </TableCell>

                                </TableRow></>
                        )
                        })}

                            </TableBody>
                        </Table>
                    </TableContainer>
                    </ThemeProvider>
                </Show>
            </Box>
        </>
    );
}