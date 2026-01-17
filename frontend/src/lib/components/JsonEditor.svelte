<script lang="ts">
    import { onMount } from "svelte";
    import { json } from "@codemirror/lang-json";

    export let value = "{}";
    export let readonly = false;
    export let error: string | null = null;

    let editorContainer: HTMLDivElement;
    let editorView: import("codemirror").EditorView | null = null;

    // Validate JSON and update error state
    function validateJson(content: string): boolean {
        try {
            JSON.parse(content);
            error = null;
            return true;
        } catch (e) {
            if (e instanceof Error) {
                error = e.message;
            } else {
                error = "Invalid JSON";
            }
            return false;
        }
    }

    onMount(async () => {
        const { EditorView, basicSetup } = await import("codemirror");
        const { EditorState } = await import("@codemirror/state");

        const updateListener = EditorView.updateListener.of((update) => {
            if (update.docChanged) {
                value = update.state.doc.toString();
                validateJson(value);
            }
        });

        const state = EditorState.create({
            doc: value,
            extensions: [
                basicSetup,
                json(),
                updateListener,
                EditorView.editable.of(!readonly),
                EditorView.theme({
                    "&": {
                        fontSize: "14px",
                        border: "1px solid var(--color-border)",
                        borderRadius: "var(--radius)",
                    },
                    ".cm-content": {
                        fontFamily: "var(--font-mono)",
                        padding: "var(--spacing-sm)",
                    },
                    ".cm-gutters": {
                        backgroundColor: "var(--color-bg)",
                        borderRight: "1px solid var(--color-border)",
                    },
                    "&.cm-focused": {
                        outline: "2px solid var(--color-primary)",
                        outlineOffset: "-1px",
                    },
                }),
            ],
        });

        editorView = new EditorView({
            state,
            parent: editorContainer,
        });

        validateJson(value);

        return () => {
            editorView?.destroy();
        };
    });

    // Update editor content when value prop changes externally
    $: if (editorView && value !== editorView.state.doc.toString()) {
        editorView.dispatch({
            changes: {
                from: 0,
                to: editorView.state.doc.length,
                insert: value,
            },
        });
    }
</script>

<div class="json-editor">
    <div bind:this={editorContainer} class="editor-container"></div>
    {#if error}
        <p class="json-error">{error}</p>
    {/if}
</div>

<style>
    .json-editor {
        width: 100%;
    }

    .editor-container {
        min-height: 200px;
    }

    .editor-container :global(.cm-editor) {
        height: 100%;
        min-height: 200px;
    }

    .json-error {
        margin: var(--spacing-xs) 0 0 0;
        padding: var(--spacing-xs) var(--spacing-sm);
        font-size: 0.875rem;
        color: var(--color-danger);
        background-color: #f8d7da;
        border-radius: var(--radius);
    }
</style>
