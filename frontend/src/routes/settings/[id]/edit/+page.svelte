<script lang="ts">
    import { goto } from "$app/navigation";
    import { updateSettings, ApiClientError } from "$lib/api";
    import type { Notification, Settings } from "$lib/types";
    import JsonEditor from "$lib/components/JsonEditor.svelte";
    import InlineNotification from "$lib/components/InlineNotification.svelte";
    import type { PageData } from "./$types";

    export let data: PageData;

    // Extract data without id for editing
    function getEditableData(settings: Settings): Record<string, unknown> {
        const { id, ...rest } = settings;
        return rest;
    }

    let jsonValue = JSON.stringify(getEditableData(data.settings), null, 2);
    let jsonError: string | null = null;
    let loading = false;
    let notification: Notification | null = null;

    async function handleSubmit() {
        if (jsonError) {
            notification = {
                type: "error",
                message: "Please fix the JSON errors before saving",
            };
            return;
        }

        loading = true;
        notification = null;

        try {
            const newData = JSON.parse(jsonValue);
            await updateSettings(data.settings.id, newData);
            notification = {
                type: "success",
                message: "Settings updated successfully",
            };
            setTimeout(() => goto("/settings"), 1000);
        } catch (e) {
            if (e instanceof ApiClientError) {
                notification = { type: "error", message: e.message };
            } else if (e instanceof SyntaxError) {
                notification = {
                    type: "error",
                    message: "Invalid JSON format",
                };
            } else {
                notification = {
                    type: "error",
                    message: "Failed to update settings",
                };
            }
        } finally {
            loading = false;
        }
    }

    function handleCancel() {
        goto("/settings");
    }
</script>

<svelte:head>
    <title>Edit Settings - Settings Manager</title>
</svelte:head>

<div class="container">
    <div class="page-header">
        <h1>Edit Settings</h1>
    </div>

    <div class="card">
        <div class="settings-id-display">
            <span class="label">ID:</span>
            <code>{data.settings.id}</code>
        </div>

        <form on:submit|preventDefault={handleSubmit}>
            <div class="form-group">
                <label for="json-editor">Settings Data (JSON)</label>
                <JsonEditor bind:value={jsonValue} bind:error={jsonError} />
            </div>

            <InlineNotification bind:notification />

            <div class="form-actions">
                <button
                    type="button"
                    class="btn-secondary"
                    on:click={handleCancel}
                    disabled={loading}
                >
                    Cancel
                </button>
                <button
                    type="submit"
                    class="btn-primary"
                    disabled={loading || !!jsonError}
                >
                    {#if loading}
                        <span class="spinner-inline"></span>
                        Saving...
                    {:else}
                        Save Changes
                    {/if}
                </button>
            </div>
        </form>
    </div>
</div>

<style>
    .settings-id-display {
        display: flex;
        align-items: center;
        gap: var(--spacing-sm);
        margin-bottom: var(--spacing-lg);
        padding-bottom: var(--spacing-md);
        border-bottom: 1px solid var(--color-border);
    }

    .settings-id-display .label {
        font-weight: 500;
        color: var(--color-text-muted);
    }

    .settings-id-display code {
        font-family: var(--font-mono);
        font-size: 0.875rem;
        background-color: var(--color-bg);
        padding: var(--spacing-xs) var(--spacing-sm);
        border-radius: var(--radius);
    }

    .btn-primary {
        display: inline-flex;
        align-items: center;
        gap: var(--spacing-xs);
    }

    .spinner-inline {
        width: 14px;
        height: 14px;
        border: 2px solid rgba(255, 255, 255, 0.3);
        border-top-color: white;
        border-radius: 50%;
        animation: spin 0.8s linear infinite;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }
</style>
