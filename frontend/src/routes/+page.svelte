<script lang="ts">
    import Categories from "$lib/components/Categories.svelte";
    import ProductList from "$lib/components/ProductList.svelte";
    import Search from "$lib/components/Search.svelte";
    import Welcome from "$lib/components/Welcome.svelte";

    let { data } = $props();

    let search = $state("");

    let currentCategory = $state("");

</script>


<div class="gap-4 flex flex-col">
    <Search bind:value={search} />
    <Categories categories={data.categories} bind:currentCategory />
    <Welcome />
    <ProductList {search} {currentCategory} products={data.products} />
</div>
