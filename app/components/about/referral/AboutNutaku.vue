<template>
  <div class="space-y-4">
    <UCard>
      <template #header>
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-gamepad-2" class="text-secondary" />
          <span class="font-medium">Nutaku</span>
          <UBadge color="secondary" variant="soft" size="sm">
            <UIcon name="i-lucide-shield-alert" class="mr-1" />
            18+ Gaming
          </UBadge>
        </div>
      </template>
      
      <div class="space-y-4">
        <UAlert color="warning" variant="soft" icon="i-lucide-shield-alert">
          <template #title>Adult Content Warning</template>
          <template #description>
            <div class="text-sm">
              <strong>18+ Only:</strong> Nutaku contains adult content and is restricted to users 18 years or older. By proceeding, you confirm you are of legal age.
            </div>
          </template>
        </UAlert>
        
        <p class="text-sm text-gray-600 dark:text-gray-400">
          Nutaku is the world's largest adult gaming platform with over 100 million players enjoying high-quality adult games for PC, Android, and iOS. Join the community and discover premium adult gaming content.
        </p>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="p-4 bg-secondary-50 dark:bg-secondary-900/20 rounded-lg border border-secondary-200 dark:border-secondary-800">
            <div class="flex items-center gap-2 mb-2">
              <UIcon name="i-lucide-gift" class="text-secondary" />
              <span class="font-medium text-secondary-800 dark:text-secondary-200">100 Free Gold</span>
            </div>
            <p class="text-xs text-secondary-700 dark:text-secondary-300">
              Get 100 Nutaku Gold free when you sign up and set up your account
            </p>
          </div>
          
          <div class="p-4 bg-secondary-50 dark:bg-secondary-900/20 rounded-lg border border-secondary-200 dark:border-secondary-800">
            <div class="flex items-center gap-2 mb-2">
              <UIcon name="i-lucide-users" class="text-secondary" />
              <span class="font-medium text-secondary-800 dark:text-secondary-200">100M+ Players</span>
            </div>
            <p class="text-xs text-secondary-700 dark:text-secondary-300">
              Join the world's largest adult gaming community with 1000+ games
            </p>
          </div>
        </div>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="p-4 bg-secondary-50 dark:bg-secondary-900/20 rounded-lg border border-secondary-200 dark:border-secondary-800">
            <div class="flex items-center gap-2 mb-2">
              <UIcon name="i-lucide-smartphone" class="text-secondary" />
              <span class="font-medium text-secondary-800 dark:text-secondary-200">Multi-Platform</span>
            </div>
            <p class="text-xs text-secondary-700 dark:text-secondary-300">
              Play on PC, Android, and iOS with cross-platform compatibility
            </p>
          </div>
          
          <div class="p-4 bg-secondary-50 dark:bg-secondary-900/20 rounded-lg border border-secondary-200 dark:border-secondary-800">
            <div class="flex items-center gap-2 mb-2">
              <UIcon name="i-lucide-star" class="text-secondary" />
              <span class="font-medium text-secondary-800 dark:text-secondary-200">Premium Quality</span>
            </div>
            <p class="text-xs text-secondary-700 dark:text-secondary-300">
              High-quality adult games with regular updates and new releases
            </p>
          </div>
        </div>
        
        <UAlert color="info" variant="soft" icon="i-lucide-info">
          <template #title>Referral Benefits</template>
          <template #description>
            <div class="text-sm">
              Both you and your friend will earn free Nutaku Gold when you join Nutaku and set up your account using our referral link!
            </div>
          </template>
        </UAlert>
        
        <div class="flex flex-col sm:flex-row gap-3">
          <UButton
            color="secondary"
            variant="solid"
            size="lg"
            icon="i-lucide-external-link"
            :loading="isLoading"
            class="flex-1"
            @click="openNutaku"
          >
            Visit Nutaku (18+)
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
          Referral ID: 1563308
        </div>
      </div>
    </UCard>
  </div>
</template>

<script setup lang="ts">
import { openUrl } from '@tauri-apps/plugin-opener'

const { showToast } = useAppToast()
const isLoading = ref(false)
const referralUrl = 'https://www.nutaku.net/fr/signup/invite/id/1563308/'

const openNutaku = async () => {
  try {
    isLoading.value = true
    await openUrl(referralUrl)
    showToast('Success', 'Nutaku opened in your default browser', 'success', 3000, 'i-lucide-external-link')
  } catch (error) {
    console.error('Failed to open Nutaku:', error)
    // Fallback to window.open if Tauri opener fails
    window.open(referralUrl, '_blank')
    showToast('Opened', 'Nutaku opened in new browser tab', 'info', 3000, 'i-lucide-external-link')
  } finally {
    isLoading.value = false
  }
}

const copyReferralLink = async () => {
  try {
    await navigator.clipboard.writeText(referralUrl)
    showToast('Copied', 'Nutaku referral link copied to clipboard', 'success', 3000, 'i-lucide-copy')
  } catch (error) {
    console.error('Failed to copy link:', error)
    showToast('Error', 'Failed to copy referral link', 'error', 3000, 'i-lucide-alert-triangle')
  }
}
</script>
