<template>
  <v-container fluid class="pa-5">

    <v-row>
      <h2 class="headline">
        Edit Label
      </h2>
    </v-row>

    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center" class="text-center" v-if="loading">
      <v-col cols="12" sm="10" md="8" xl="6">
        <v-progress-circular
          :size="50"
          color="primary"
          indeterminate
        />
      </v-col>
    </v-row>

    <v-row class="pa-5" v-if="label">
      <label-details :projectFullPath="projectFullPath" :label="label" />
    </v-row>

  </v-container>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';
import LabelDetails from '@/ui/components/collaboration/label_details.vue';
import { Label } from '@/api/graphql/model';

export default VueApp.extend({
  name: 'ProjectEditLabelView',
  components: {
    LabelDetails,
  },
  data() {
    return {
      loading: false,
      error: '',
      label: null as Label | null,
    };
  },
  computed: {
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
  },
  created() {
    this.fetchData();
  },
  methods: {
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.label = await this.$collaborationService.fetchLabel(this.$route.params.labelId);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
  },
});
</script>

<style lang="scss" scoped>
</style>
