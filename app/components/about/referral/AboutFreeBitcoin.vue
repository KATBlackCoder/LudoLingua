<template>
  <div class="space-y-6">
    <!-- Header Section -->
    <UCard>
      <template #header>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="p-2 bg-warning-50 dark:bg-warning-900/20 rounded-lg">
              <UIcon name="i-lucide-bitcoin" class="text-warning w-5 h-5" />
            </div>
            <div>
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white">FreeBitcoin</h3>
              <p class="text-sm text-gray-500 dark:text-gray-400">Cryptocurrency earning platform</p>
            </div>
          </div>
          <UBadge color="warning" variant="soft" size="sm">
            <UIcon name="i-lucide-coins" class="w-3 h-3 mr-1" />
            Cryptocurrency
          </UBadge>
        </div>
      </template>

      <div class="space-y-6">
        <!-- Description -->
        <div class="text-sm text-gray-600 dark:text-gray-400 leading-relaxed">
          FreeBitcoin is a legitimate cryptocurrency earning platform where you can win free Bitcoin every hour through provably fair games and earn interest on your deposits.
        </div>

        <!-- Features Grid -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <UCard class="p-4 bg-warning-50 dark:bg-warning-900/20 border border-warning-200 dark:border-warning-800">
            <div class="flex items-start gap-3">
              <div class="p-2 bg-warning-100 dark:bg-warning-900/30 rounded-lg">
                <UIcon name="i-lucide-gift" class="text-warning w-4 h-4" />
              </div>
              <div class="flex-1">
                <h4 class="font-semibold text-warning-800 dark:text-warning-200 mb-1">Free Bitcoin Hourly</h4>
                <p class="text-xs text-warning-700 dark:text-warning-300">
                  Win up to $200 in free Bitcoin every hour with our free-to-play game
                </p>
              </div>
            </div>
          </UCard>
          
          <UCard class="p-4 bg-warning-50 dark:bg-warning-900/20 border border-warning-200 dark:border-warning-800">
            <div class="flex items-start gap-3">
              <div class="p-2 bg-warning-100 dark:bg-warning-900/30 rounded-lg">
                <UIcon name="i-lucide-trending-up" class="text-warning w-4 h-4" />
              </div>
              <div class="flex-1">
                <h4 class="font-semibold text-warning-800 dark:text-warning-200 mb-1">4.08% Annual Interest</h4>
                <p class="text-xs text-warning-700 dark:text-warning-300">
                  Earn daily compounded interest on your Bitcoin deposits
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
          title="Referral Benefits"
          description="By using our referral link, you'll get a bonus when you sign up and start earning Bitcoin. We both benefit from the referral program!"
        />
        
        <!-- Action Buttons -->
        <div class="flex flex-col sm:flex-row gap-3">
          <UButton
            color="warning"
            variant="solid"
            size="lg"
            icon="i-lucide-external-link"
            class="flex-1"
            :to="referralUrl"
            target="_blank"
          >
            Visit FreeBitcoin
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
            <span class="text-xs text-gray-500 dark:text-gray-400 font-mono">Referral ID: 10431332</span>
          </div>
        </div>
      </div>
    </UCard>
  </div>
</template>

<script setup lang="ts">
const { showToast } = useAppToast()
const referralUrl = 'https://freebitco.in/?r=10431332'

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
