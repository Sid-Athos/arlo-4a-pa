import { RiMediaVideoAddFill } from "solid-icons/ri";

import {useNavigate} from "@solidjs/router";

export default function NewMeeting() {
    const navigate = useNavigate();

    const handleMeet = () => {
        const code = 1;
        navigate(`/${code}`);
    };
    return (
        <button
            onClick={handleMeet}
            className="flex items-center gap-3 justify-center bg-blue-500 text-white rounded px-3 py-3 font-bold flex-none"
        >
            <RiMediaVideoAddFill/>
            New meeting
        </button>
    );
}