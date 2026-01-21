<script lang="ts">
    import { onMount } from "svelte";
    import { getAllSettings, deleteSettings } from "$lib/api";
    import type { Settings, Notification, PaginationInfo } from "$lib/types";
    import SettingsCard from "$lib/components/SettingsCard.svelte";
    import ConfirmDialog from "$lib/components/ConfirmDialog.svelte";
    import InlineNotification from "$lib/components/InlineNotification.svelte";

    let items: Settings[] = $state([]);
    let pagination: PaginationInfo | null = $state(null);
    let loading = $state(true);
    let notification: Notification | null = $state(null);
    let currentPage = $state(0);
    let pageSize = 10;

    // Delete confirmation dialog state
    let deleteDialogOpen = $state(false);
    let deleteTargetId: string | null = null;
    let deleteLoading = $state(false);

    let totalPages: number | null = $state(null);
    let hasNextPage: boolean = $state(false);
    let hasPrevPage: boolean = $state(false);

    async function loadSettings() {
        loading = true;
        try {
            const result = await getAllSettings(
                pageSize,
                currentPage * pageSize,
            );
            items = result.items;
            pagination = result.pagination;
            totalPages = Math.ceil(pagination.total / pageSize);
            hasNextPage = (currentPage + 1) * pageSize < pagination.total;
            hasPrevPage = currentPage > 0;
        } catch (e) {
            notification = {
                type: "error",
                message:
                    e instanceof Error ? e.message : "Failed to load settings",
            };
        } finally {
            loading = false;
        }
    }

    function openDeleteDialog(id: string) {
        deleteTargetId = id;
        deleteDialogOpen = true;
    }

    async function confirmDelete() {
        if (!deleteTargetId) return;

        deleteLoading = true;
        try {
            await deleteSettings(deleteTargetId);
            notification = {
                type: "success",
                message: "Settings deleted successfully",
            };
            deleteDialogOpen = false;
            deleteTargetId = null;
            if (items.length == 1 && currentPage > 0) {
                // added to account for deleting the "page" we are on
                currentPage -= 1;
            }
            await loadSettings();
        } catch (e) {
            notification = {
                type: "error",
                message:
                    e instanceof Error
                        ? e.message
                        : "Failed to delete settings",
            };
        } finally {
            deleteLoading = false;
        }
    }

    function cancelDelete() {
        deleteDialogOpen = false;
        deleteTargetId = null;
    }

    function nextPage() {
        if (pagination && (currentPage + 1) * pageSize < pagination.total) {
            currentPage++;
            loadSettings();
        }
    }

    function prevPage() {
        if (currentPage > 0) {
            currentPage--;
            loadSettings();
        }
    }

    onMount(() => {
        loadSettings();
    });
</script>

<svelte:head>
    <title>Settings - Settings Manager</title>
</svelte:head>

<div class="container">
    <div class="page-header">
        <h1>Settings</h1>
        <a href="/new" class="btn-primary">Create New</a>
    </div>

    <InlineNotification bind:notification />

    {#if loading}
        <div class="loading">
            <div class="spinner"></div>
            <span>Loading settings...</span>
        </div>
    {:else if items.length === 0}
        <div class="card empty-state">
            <p>No settings found.</p>
            <a href="/new" class="btn-primary">Create your first settings</a>
        </div>
    {:else}
        <div class="settings-list">
            {#each items as settings (settings.id)}
                <SettingsCard {settings} onDelete={openDeleteDialog} />
            {/each}
        </div>

        {#if pagination && pagination.total > pageSize}
            <div class="pagination">
                <button
                    type="button"
                    class="btn-secondary"
                    onclick={prevPage}
                    disabled={!hasPrevPage}
                >
                    Previous
                </button>
                <span class="pagination-info">
                    Page {currentPage + 1} of {totalPages} ({pagination.total} total)
                </span>
                <button
                    type="button"
                    class="btn-secondary"
                    onclick={nextPage}
                    disabled={!hasNextPage}
                >
                    Next
                </button>
            </div>
        {/if}
    {/if}
</div>

<ConfirmDialog
    open={deleteDialogOpen}
    title="Delete Settings"
    message="Are you sure you want to delete this settings object? This action cannot be undone."
    confirmText="Delete"
    danger={true}
    loading={deleteLoading}
    onConfirm={confirmDelete}
    onCancel={cancelDelete}
/>

<style>
    .settings-list {
        display: flex;
        flex-direction: column;
        gap: var(--spacing-md);
    }

    .pagination {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: var(--spacing-md);
        margin-top: var(--spacing-lg);
        padding-top: var(--spacing-lg);
        border-top: 1px solid var(--color-border);
    }

    .pagination-info {
        color: var(--color-text-muted);
        font-size: 0.875rem;
    }
</style>
