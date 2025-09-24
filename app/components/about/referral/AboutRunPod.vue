<template>
  <div class="space-y-6">
    <!-- Header Section -->
    <UCard>
      <template #header>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="p-2 bg-info-50 dark:bg-info-900/20 rounded-lg">
              <UIcon name="i-lucide-cloud" class="text-info w-5 h-5" />
            </div>
            <div>
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white">RunPod</h3>
              <p class="text-sm text-gray-500 dark:text-gray-400">AI infrastructure platform</p>
            </div>
          </div>
          <UBadge color="info" variant="soft" size="sm">
            <UIcon name="i-lucide-zap" class="w-3 h-3 mr-1" />
            AI Infrastructure
          </UBadge>
        </div>
      </template>

      <div class="space-y-6">
        <!-- Description -->
        <div class="text-sm text-gray-600 dark:text-gray-400 leading-relaxed">
          RunPod is the leading AI infrastructure platform for training, fine-tuning, and deploying machine learning models. Get access to powerful GPU clusters with pay-per-use pricing.
        </div>

        <!-- Features Grid -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <UCard class="p-4 bg-info-50 dark:bg-info-900/20 border border-info-200 dark:border-info-800">
            <div class="flex items-start gap-3">
              <div class="p-2 bg-info-100 dark:bg-info-900/30 rounded-lg">
                <UIcon name="i-lucide-cpu" class="text-info w-4 h-4" />
              </div>
              <div class="flex-1">
                <h4 class="font-semibold text-info-800 dark:text-info-200 mb-1">GPU Clusters</h4>
                <p class="text-xs text-info-700 dark:text-info-300">
                  Deploy multi-node GPU clusters in minutes across 31 global regions
                </p>
              </div>
            </div>
          </UCard>
          
          <UCard class="p-4 bg-info-50 dark:bg-info-900/20 border border-info-200 dark:border-info-800">
            <div class="flex items-start gap-3">
              <div class="p-2 bg-info-100 dark:bg-info-900/30 rounded-lg">
                <UIcon name="i-lucide-rocket" class="text-info w-4 h-4" />
              </div>
              <div class="flex-1">
                <h4 class="font-semibold text-info-800 dark:text-info-200 mb-1">Serverless AI</h4>
                <p class="text-xs text-info-700 dark:text-info-300">
                  Instant AI workloads with no setup, scaling, or idle costs
                </p>
              </div>
            </div>
          </UCard>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <UCard class="p-4 bg-info-50 dark:bg-info-900/20 border border-info-200 dark:border-info-800">
            <div class="flex items-start gap-3">
              <div class="p-2 bg-info-100 dark:bg-info-900/30 rounded-lg">
                <UIcon name="i-lucide-dollar-sign" class="text-info w-4 h-4" />
              </div>
              <div class="flex-1">
                <h4 class="font-semibold text-info-800 dark:text-info-200 mb-1">Cost Effective</h4>
                <p class="text-xs text-info-700 dark:text-info-300">
                  Pay only for what you use with automatic scaling and zero idle costs
                </p>
              </div>
            </div>
          </UCard>
          
          <UCard class="p-4 bg-info-50 dark:bg-info-900/20 border border-info-200 dark:border-info-800">
            <div class="flex items-start gap-3">
              <div class="p-2 bg-info-100 dark:bg-info-900/30 rounded-lg">
                <UIcon name="i-lucide-shield-check" class="text-info w-4 h-4" />
              </div>
              <div class="flex-1">
                <h4 class="font-semibold text-info-800 dark:text-info-200 mb-1">Enterprise Ready</h4>
                <p class="text-xs text-info-700 dark:text-info-300">
                  99.9% uptime with SOC2, HIPAA and GDPR certifications in progress
                </p>
              </div>
            </div>
          </UCard>
        </div>

        <!-- Referral Benefits Alert -->
        <UAlert 
          color="info" 
          variant="soft" 
          icon="i-lucide-info"
          title="Referral Bonus"
          description="Sign up with our referral link and get a random credit bonus between $5 and $500 when you spend your first $10 on RunPod!"
        />
        
        <!-- Action Buttons -->
        <div class="flex flex-col sm:flex-row gap-3">
          <UButton
            color="info"
            variant="solid"
            size="lg"
            icon="i-lucide-external-link"
            class="flex-1"
            :to="referralUrl"
            target="_blank"
          >
            Visit RunPod
          </UButton>
          
          <UButton
            color="neutral"
            variant="outline"
            size="lg"
            icon="i-lucide-copy"
            @click="copyReferralLink"
          >
            Copy Link
          </UButton>
        </div>
        
        <!-- Referral ID -->
        <div class="text-center">
          <div class="inline-flex items-center gap-2 px-3 py-1 bg-gray-100 dark:bg-gray-800 rounded-full">
            <UIcon name="i-lucide-hash" class="text-gray-500 w-3 h-3" />
            <span class="text-xs text-gray-500 dark:text-gray-400 font-mono">Referral ID: vsxy2ugb</span>
          </div>
        </div>
      </div>
    </UCard>
  </div>
</template>

<script setup lang="ts">
const { showToast } = useAppToast()
const referralUrl = 'https://runpod.io?ref=vsxy2ugb'

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
