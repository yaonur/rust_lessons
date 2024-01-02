<script lang="ts">
    import {sum,greet} from '$lib/pkg/greet'

</script>
<p>{sum(1,3)}</p>
<button on:click={()=>greet('onur')}>Greet</button>