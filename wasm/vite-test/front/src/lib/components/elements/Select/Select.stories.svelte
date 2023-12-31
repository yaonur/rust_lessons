<script>
    import { Meta, Template, Story } from '@storybook/addon-svelte-csf';
    import Select from './Select.svelte';
</script>

<Meta title="elements/select" component={Select}/>

<Template let:args>
    <Select {...args}>Some Select</Select>
</Template>

<Story name="Select Slot" args={{intent:"primary",size:"small"}}></Story>

<Story name="Select Secondary" args={{intent:"secondary",size:"medium" }}/>