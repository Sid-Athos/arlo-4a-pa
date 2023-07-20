import {createCodeMirror, createEditorControlledValue} from "solid-codemirror";
import { createSignal } from "solid-js";
import {EditorView, lineNumbers} from "@codemirror/view";
import {Box, Container, Grid, Paper, styled} from "@suid/material";

import {
    autocompletion,
    closeBrackets,
    closeBracketsKeymap,
    completionKeymap,
} from '@codemirror/autocomplete';
import {
    defaultKeymap,
    history,
    historyKeymap,
    indentWithTab,
} from '@codemirror/commands';
import {bracketMatching, indentOnInput} from '@codemirror/language';
import {EditorState} from '@codemirror/state';
import {
    crosshairCursor,
    drawSelection,
    dropCursor,
    highlightSpecialChars,
    keymap,
    rectangularSelection,
} from '@codemirror/view';

export default function Editor(){
    const [code, setCode] = createSignal("console.log('hello world!')");
    const [showLineNumber, setShowLineNumber] = createSignal(true);
    const { ref, editorView, createExtension } = createCodeMirror({ onValueChange: setCode });

    const theme = EditorView.theme(
    {
        '&': {
        textAlign: 'left',
            background: '#333',
            color:'#fff'
    },
        '.cm-content': {
        textAlign: 'left',
    },
        '.cm-gutters': {
        backgroundColor: '#333',
            border: 'none',
    },
        '.cm-lineNumbers': {
        position: 'sticky',
            flexDirection: 'column',
            flexShrink: 0,
    },
        '.cm-lineNumbers .cm-gutterElement': {
            backgroundColor: '#333',
        textAlign: 'right',
            padding: '0 16px 0 8px',
            lineHeight: '21px',
    },
        '.cm-line': {
        padding: '0 2px 0 8px',
    },
        '.cm-cursor': {
        borderLeftWidth: '2px',
            height: '21px',
            transform: 'translateY(-10%)',
    }
    },{dark: true});

    // Add a static custom theme
    createExtension(theme);
    const Item = styled(Paper)(({ theme }) => ({
        ...theme.typography.body2,
        padding: theme.spacing(1),
        textAlign: "center",
        backgroundColor:'#333',
        color: theme.palette.text.secondary,
    }));
    createExtension(() => showLineNumber() ? lineNumbers() : []);
    createExtension(highlightSpecialChars({
        addSpecialChars: new RegExp("function")
    }))
    createExtension(bracketMatching())
    createExtension(closeBrackets())
    createExtension(history())
    createExtension(EditorState.allowMultipleSelections.of(true))
    createEditorControlledValue(editorView, code);

    return (
        <>
            <Container maxWidth="sm" sx={{paddingTop: '50px'}}>
            <Box sx={{ flexGrow: 1 }}>
                <Grid container spacing={2}>
                    <Item>
                        <div ref={ref} sx={{minWidth:'500px'}}/>
                    </Item>
                </Grid>
            </Box>
            </Container>

        </>
    )
}