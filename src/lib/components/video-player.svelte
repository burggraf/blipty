<script lang="ts">
	export let src: string;
</script>

<div class="w-full aspect-video bg-black">
	<video class="w-full h-full" controls autoplay {src}>
		<track kind="captions" />
	</video>
</div>
