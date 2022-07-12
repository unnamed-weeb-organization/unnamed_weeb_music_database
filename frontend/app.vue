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
	<div class="flex flex-col h-full">
	</div>
</template>

<style lang="scss">
	@use "assets/css/fonts.css";
	@use "assets/css/themes.scss";

	html,
	body,
	#__nuxt {
		height: 100vh;
		font-family: "Nunito Sans";
		background-color: var(--background);
		color: var(--300);
	}
</style>
