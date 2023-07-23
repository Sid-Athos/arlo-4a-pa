import {createStore} from "solid-js/store";

const [user, setUser] = createStore({});

const getUser = () => {
    return user
}

const saveUser = (toRegister) => {
    setUser(toRegister)
}

export const UserStore = {
    get: getUser,
    save: saveUser
}