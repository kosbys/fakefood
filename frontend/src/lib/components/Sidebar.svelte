<script>
    let { children } = $props();
</script>

<div class="drawer drawer-end">
  <input id="cart-drawer" type="checkbox" class="drawer-toggle" />
  <div class="drawer-content max-w-2/4 items-center m-auto">
    {@render children()}
  </div>
  <div class="drawer-side">
    <label for="cart-drawer" aria-label="close sidebar" class="drawer-overlay"></label>
    <ul class="menu bg-base-200 min-h-full w-80 p-4">
        <!-- load items from cart -->
      <!-- Sidebar content here -->
      <li><a>Sidebar Item 1</a></li>
      <li><a>Sidebar Item 2</a></li>
    </ul>
  </div>
</div>
