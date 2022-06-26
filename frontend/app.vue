<script setup lang="ts">
	useHead({
		title: "unnamed_weeb_database",
		meta: [
			{
				name: "description",
				content: "Weeb related music metadata lookup database.",
			},
		],
	});
</script>

<template>
	<div class="flex flex-col h-full">Hello World!</div>
</template>

<style lang="scss">
	@import "assets/css/fonts.css";

	html,
	body {
		height: 100vh;
		font-family: "Nunito Sans";
		background-color: var(--background);
		color: var(--300);
	}
</style>
