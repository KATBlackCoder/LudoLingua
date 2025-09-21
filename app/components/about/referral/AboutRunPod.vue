<template>
  <div class="space-y-4">
    <UCard>
      <template #header>
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-cloud" class="text-info" />
          <span class="font-medium">RunPod</span>
          <UBadge color="info" variant="soft" size="sm">
            <UIcon name="i-lucide-zap" class="mr-1" />
            AI Infrastructure
          </UBadge>
        </div>
      </template>
      
      <div class="space-y-4">
        <p class="text-sm text-gray-600 dark:text-gray-400">
          RunPod is the leading AI infrastructure platform for training, fine-tuning, and deploying machine learning models. Get access to powerful GPU clusters with pay-per-use pricing.
        </p>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="p-4 bg-info-50 dark:bg-info-900/20 rounded-lg border border-info-200 dark:border-info-800">
            <div class="flex items-center gap-2 mb-2">
              <UIcon name="i-lucide-cpu" class="text-info" />
              <span class="font-medium text-info-800 dark:text-info-200">GPU Clusters</span>
            </div>
            <p class="text-xs text-info-700 dark:text-info-300">
              Deploy multi-node GPU clusters in minutes across 31 global regions
            </p>
          </div>
          
          <div class="p-4 bg-info-50 dark:bg-info-900/20 rounded-lg border border-info-200 dark:border-info-800">
            <div class="flex items-center gap-2 mb-2">
              <UIcon name="i-lucide-rocket" class="text-info" />
              <span class="font-medium text-info-800 dark:text-info-200">Serverless AI</span>
            </div>
            <p class="text-xs text-info-700 dark:text-info-300">
              Instant AI workloads with no setup, scaling, or idle costs
            </p>
          </div>
        </div>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="p-4 bg-info-50 dark:bg-info-900/20 rounded-lg border border-info-200 dark:border-info-800">
            <div class="flex items-center gap-2 mb-2">
              <UIcon name="i-lucide-dollar-sign" class="text-info" />
              <span class="font-medium text-info-800 dark:text-info-200">Cost Effective</span>
            </div>
            <p class="text-xs text-info-700 dark:text-info-300">
              Pay only for what you use with automatic scaling and zero idle costs
            </p>
          </div>
          
          <div class="p-4 bg-info-50 dark:bg-info-900/20 rounded-lg border border-info-200 dark:border-info-800">
            <div class="flex items-center gap-2 mb-2">
              <UIcon name="i-lucide-shield-check" class="text-info" />
              <span class="font-medium text-info-800 dark:text-info-200">Enterprise Ready</span>
            </div>
            <p class="text-xs text-info-700 dark:text-info-300">
              99.9% uptime with SOC2, HIPAA and GDPR certifications in progress
            </p>
          </div>
        </div>
        
        <UAlert color="info" variant="soft" icon="i-lucide-info">
          <template #title>Referral Bonus</template>
          <template #description>
            <div class="text-sm">
              Sign up with our referral link and get a random credit bonus between $5 and $500 when you spend your first $10 on RunPod!
            </div>
          </template>
        </UAlert>
        
        <div class="flex flex-col sm:flex-row gap-3">
          <UButton
            color="info"
            variant="solid"
            size="lg"
            icon="i-lucide-external-link"
            :loading="isLoading"
            class="flex-1"
            @click="openRunPod"
          >
            Visit RunPod
          </UButton>
          
          <UButton
            color="neutral"
            variant="outline"
            size="lg"
            icon="i-lucide-copy"
            :disabled="isLoading"
            @click="copyReferralLink"
          >
            Copy Link
          </UButton>
        </div>
        
        <div class="text-xs text-gray-500 dark:text-gray-400 text-center">
          Referral ID: vsxy2ugb
        </div>
      </div>
    </UCard>
  </div>
</template>

<script setup lang="ts">
import { openUrl } from '@tauri-apps/plugin-opener'

const { showToast } = useAppToast()
const isLoading = ref(false)
const referralUrl = 'https://runpod.io?ref=vsxy2ugb'

const openRunPod = async () => {
  try {
    isLoading.value = true
    await openUrl(referralUrl)
    showToast('Success', 'RunPod opened in your default browser', 'success', 3000, 'i-lucide-external-link')
  } catch (error) {
    console.error('Failed to open RunPod:', error)
    // Fallback to window.open if Tauri opener fails
    window.open(referralUrl, '_blank')
    showToast('Opened', 'RunPod opened in new browser tab', 'info', 3000, 'i-lucide-external-link')
  } finally {
    isLoading.value = false
  }
}

const copyReferralLink = async () => {
  try {
    await navigator.clipboard.writeText(referralUrl)
    showToast('Copied', 'RunPod referral link copied to clipboard', 'success', 3000, 'i-lucide-copy')
  } catch (error) {
    console.error('Failed to copy link:', error)
    showToast('Error', 'Failed to copy referral link', 'error', 3000, 'i-lucide-alert-triangle')
  }
}
</script>
