import { CodeMirror } from "@solid-codemirror/codemirror";
import { basicSetup } from "codemirror";
import { java } from "@codemirror/lang-java";

export default function CodeEditor() {
    return <CodeMirror extensions={[basicSetup, java()]} />;
}