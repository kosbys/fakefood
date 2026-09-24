<script>
    let { currentCategory = $bindable(""), categories } = $props();
    // get categories from server
</script>

<div class="flex justify-between">
    {#each categories as category}
        <div>cat</div>
    {/each}
    <button class="btn btn-soft">Default</button>
    <button class="btn btn-soft">Default</button>
    <button class="btn btn-soft">Default</button>
    <button class="btn btn-soft">Default</button>
    <button class="btn btn-soft">Default</button>
    <button class="btn btn-soft">Default</button>
    <button class="btn btn-soft">Default</button>
    <button class="btn btn-soft">Default</button>
    <button class="btn btn-soft">Default</button>
</div>
