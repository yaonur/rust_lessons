<script>
    import { Meta, Template, Story } from '@storybook/addon-svelte-csf';
    import Button from './Button.svelte';
</script>

<Meta title="elements/button" component={Button}/>

<Template let:args>
    <Button {...args}>Some Button</Button>
</Template>

<Story name="Button Slot" args={{intent:"primary",size:"small"}}></Story>

<Story name="Button Secondary" args={{intent:"secondary",size:"medium" }}/>