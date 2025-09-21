<template>
  <div class="space-y-4">
    <UCard>
      <template #header>
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-bitcoin" class="text-warning" />
          <span class="font-medium">FreeBitcoin</span>
          <UBadge color="warning" variant="soft" size="sm">
            <UIcon name="i-lucide-coins" class="mr-1" />
            Cryptocurrency
          </UBadge>
        </div>
      </template>
      
      <div class="space-y-4">
        <p class="text-sm text-gray-600 dark:text-gray-400">
          FreeBitcoin is a legitimate cryptocurrency earning platform where you can win free Bitcoin every hour through provably fair games and earn interest on your deposits.
        </p>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="p-4 bg-warning-50 dark:bg-warning-900/20 rounded-lg border border-warning-200 dark:border-warning-800">
            <div class="flex items-center gap-2 mb-2">
              <UIcon name="i-lucide-gift" class="text-warning" />
              <span class="font-medium text-warning-800 dark:text-warning-200">Free Bitcoin Hourly</span>
            </div>
            <p class="text-xs text-warning-700 dark:text-warning-300">
              Win up to $200 in free Bitcoin every hour with our free-to-play game
            </p>
          </div>
          
          <div class="p-4 bg-warning-50 dark:bg-warning-900/20 rounded-lg border border-warning-200 dark:border-warning-800">
            <div class="flex items-center gap-2 mb-2">
              <UIcon name="i-lucide-trending-up" class="text-warning" />
              <span class="font-medium text-warning-800 dark:text-warning-200">4.08% Annual Interest</span>
            </div>
            <p class="text-xs text-warning-700 dark:text-warning-300">
              Earn daily compounded interest on your Bitcoin deposits
            </p>
          </div>
        </div>
        
        <UAlert color="info" variant="soft" icon="i-lucide-info">
          <template #title>Referral Benefits</template>
          <template #description>
            <div class="text-sm">
              By using our referral link, you'll get a bonus when you sign up and start earning Bitcoin. We both benefit from the referral program!
            </div>
          </template>
        </UAlert>
        
        <div class="flex flex-col sm:flex-row gap-3">
          <UButton
            color="warning"
            variant="solid"
            size="lg"
            icon="i-lucide-external-link"
            :loading="isLoading"
            class="flex-1"
            @click="openFreeBitcoin"
          >
            Visit FreeBitcoin
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
          Referral ID: 10431332
        </div>
      </div>
    </UCard>
  </div>
</template>

<script setup lang="ts">
import { openUrl } from '@tauri-apps/plugin-opener'

const { showToast } = useAppToast()
const isLoading = ref(false)
const referralUrl = 'https://freebitco.in/?r=10431332'

const openFreeBitcoin = async () => {
  try {
    isLoading.value = true
    await openUrl(referralUrl)
    showToast('Success', 'FreeBitcoin opened in your default browser', 'success', 3000, 'i-lucide-external-link')
  } catch (error) {
    console.error('Failed to open FreeBitcoin:', error)
    // Fallback to window.open if Tauri opener fails
    window.open(referralUrl, '_blank')
    showToast('Opened', 'FreeBitcoin opened in new browser tab', 'info', 3000, 'i-lucide-external-link')
  } finally {
    isLoading.value = false
  }
}

const copyReferralLink = async () => {
  try {
    await navigator.clipboard.writeText(referralUrl)
    showToast('Copied', 'FreeBitcoin referral link copied to clipboard', 'success', 3000, 'i-lucide-copy')
  } catch (error) {
    console.error('Failed to copy link:', error)
    showToast('Error', 'Failed to copy referral link', 'error', 3000, 'i-lucide-alert-triangle')
  }
}
</script>
