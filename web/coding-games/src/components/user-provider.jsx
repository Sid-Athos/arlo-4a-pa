import {createContext, createSignal, useContext} from "solid-js";

const UserContext = createContext();


export function UserProvider(props) {
    const [token, setToken] = createSignal(props.token || ""),
        user = [
            token,
            {
                updateToken(value) {
                    setToken(value);
                },
            }
        ];

    return (
        <UserContext.Provider value={user}>
            {props.children}
        </UserContext.Provider>
    );
}

export function getUser() { return useContext(UserContext); }