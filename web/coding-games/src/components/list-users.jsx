import {createSignal, onMount, Show} from "solid-js";
import {UserService} from "../utils/services/user-service";
import {Box, List, ListItem, ListItemButton, ListItemIcon, ListItemText} from "@suid/material";
import SearchComponent from "./search-bar.jsx"
import createSvgIcon from "@suid/material/utils/createSvgIcon";


export default function ListUsers() {
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

    const addFriend = async (recipientId) => {
        await UserService.addFriend({recipient_id: recipientId})
    }
    return (
        <>
            <Box sx={{flexGrow: 1, m: 8}}>
                <SearchComponent setUserList={setUserList}></SearchComponent>
                <Show when={userList().length > 0}>
                    <List sx={{color:"white"}}>
                        <ListItem disablePadding >
                                <ListItemText primary={"Pseudo"} sx={{maxWidth:'250px'}}/>
                                <ListItemText primary={"Experience"} sx={{maxWidth:'250px'}}/>
                                <ListItemText primary={"Level"} sx={{maxWidth:'250px'}}/>
                                <ListItemText primary={"Action"} sx={{maxWidth:'250px'}}/>
                            </ListItem>

                    </List>
                    <List sx={{color:"white"}}>
                        {userList().map(user => {
                            return (<><ListItem disablePadding>
                                    <ListItemText primary={user.pseudo} sx={{maxWidth:'250px'}}/>
                                    <ListItemText primary={user.experience} sx={{maxWidth:'250px'}}/>
                                    <ListItemText primary={user.level} sx={{maxWidth:'250px'}}/>
                                    <ListItemButton onClick={() => addFriend(user.id)} sx={{maxWidth:'250px'}}>
                                            <PlusIcon/>
                                    </ListItemButton>
                            </ListItem></>)

                        })}
                    </List>

                </Show>
            </Box>
        </>
    );
}