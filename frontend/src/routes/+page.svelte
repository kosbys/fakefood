<script>
    import Categories from "$lib/components/Categories.svelte";
    import ProductList from "$lib/components/ProductList.svelte";
    import Search from "$lib/components/Search.svelte";
    import Welcome from "$lib/components/Welcome.svelte";
</script>


<div class="gap-4 flex flex-col">
    <Search />
    <Categories />
    <Welcome />
    <ProductList />
</div>
