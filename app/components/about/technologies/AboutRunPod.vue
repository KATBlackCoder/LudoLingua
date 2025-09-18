<template>
  <section>
    <h3 class="text-lg font-medium mb-4">RunPod Configuration</h3>
    
    <UAlert
      color="info"
      variant="soft"
      icon="i-lucide-cloud"
      class="mb-4"
      title="Cloud-based Ollama"
    >
      <template #description>
        <div class="text-sm">
          RunPod provides cloud GPU instances that can run Ollama models. This is ideal if you don't have a powerful local GPU or want to scale your translation workload.
        </div>
      </template>
    </UAlert>

    <UCard class="mb-4">
      <template #header>
        <div class="font-medium">Setup Instructions</div>
      </template>
      <div class="space-y-4 text-sm">
        <div>
          <div class="font-medium mb-2">1. Create a RunPod Instance</div>
          <div class="text-muted mb-2">Choose a GPU instance with sufficient VRAM for your model:</div>
          <UAlert color="error" variant="soft" icon="i-lucide-info" class="mb-3">
            <template #title>These are suggestions, not requirements</template>
            <template #description>
              The GPU choices below are recommendations based on model sizes. You can choose any instance that meets your needs and budget. Smaller instances work fine with smaller models.
            </template>
          </UAlert>
          <ul class="list-disc pl-5 space-y-1">
            <li><strong>RTX 4090 (24GB):</strong> For 13B+ models</li>
            <li><strong>RTX 3080/4080 (12GB):</strong> For 7B-8B models</li>
            <li><strong>RTX 3060 (12GB):</strong> For 7B models (Q4 quantization)</li>
          </ul>
        </div>

        <USeparator />

        <div>
          <div class="font-medium mb-2">2. Container Start Command</div>
          <div class="text-muted mb-2">Use this command to automatically install and configure Ollama:</div>
          <UCard class="bg-gray-50 dark:bg-gray-800">
            <div class="flex items-start justify-between gap-2">
              <pre class="text-xs overflow-x-auto flex-1"><code>{{ containerCommand }}</code></pre>
              <UTooltip text="Copy to clipboard" :content="{ side: 'left' }">
                <UButton
                  :color="copied ? 'success' : 'neutral'"
                  variant="link"
                  size="sm"
                  :icon="copied ? 'i-lucide-copy-check' : 'i-lucide-copy'"
                  aria-label="Copy to clipboard"
                  @click="copy(containerCommand)"
                />
              </UTooltip>
            </div>
          </UCard>
          <UAlert color="error" variant="soft" icon="i-lucide-info" class="mt-2">
            <template #title>Model choice is entirely up to you</template>
            <template #description>
              Replace <code>qwen2.5:14b</code> with any model you prefer (e.g., <code>qwen2.5:7b</code>, <code>llama3.1:8b</code>, <code>mistral:7b</code>). This is just an example - choose whatever model works best for your translation needs and fits your pod's VRAM.
            </template>
          </UAlert>
        </div>

        <USeparator />

        <div>
          <div class="font-medium mb-2">3. Get Your Pod URL</div>
          <div class="text-muted mb-2">Your RunPod endpoint will be:</div>
          <UCard class="bg-gray-50 dark:bg-gray-800">
            <code class="text-sm">https://{POD_ID}-11434.proxy.runpod.net</code>
          </UCard>
          <div class="text-muted mt-1">Replace <code>{POD_ID}</code> with your actual pod ID from RunPod dashboard.</div>
        </div>

        <USeparator />

        <div>
          <div class="font-medium mb-2">4. Configure in LudoLingua</div>
          <div class="text-muted mb-2">In Settings, set:</div>
          <ul class="list-disc pl-5 space-y-1">
            <li><strong>Provider:</strong> RunPod</li>
            <li><strong>Base URL:</strong> <code>https://{POD_ID}-11434.proxy.runpod.net</code></li>
            <li><strong>Model:</strong> Select the model you pulled (e.g., qwen2.5:14b)</li>
          </ul>
        </div>
      </div>
    </UCard>

    <UCard class="mb-4">
      <template #header>
        <div class="font-medium">Recommended RunPod Templates</div>
      </template>
      <div class="space-y-3 text-sm">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
          <UCard>
            <template #header>
              <div class="flex items-center gap-2">
                <UBadge color="primary" variant="soft">High Performance</UBadge>
                <span class="font-medium">RTX 4090</span>
              </div>
            </template>
            <div class="space-y-2">
              <div><strong>VRAM:</strong> 24GB</div>
              <div><strong>Models:</strong> 13B+ models, Qwen2.5:14b, Llama3.1:70b</div>
              <div><strong>Cost:</strong> ~$0.79/hour</div>
              <div><strong>Best for:</strong> Large projects, high-quality translations</div>
            </div>
          </UCard>
          <UCard>
            <template #header>
              <div class="flex items-center gap-2">
                <UBadge color="neutral" variant="soft">Balanced</UBadge>
                <span class="font-medium">RTX 3080/4080</span>
              </div>
            </template>
            <div class="space-y-2">
              <div><strong>VRAM:</strong> 12GB</div>
              <div><strong>Models:</strong> 7B-8B models, Qwen2.5:7b, Llama3.1:8b</div>
              <div><strong>Cost:</strong> ~$0.34/hour</div>
              <div><strong>Best for:</strong> Most translation tasks, cost-effective</div>
            </div>
          </UCard>
        </div>
      </div>
    </UCard>

    <UCard class="mb-4">
      <template #header>
        <div class="font-medium">Cost Optimization Tips</div>
      </template>
      <div class="space-y-3 text-sm">
        <ul class="list-disc pl-5 space-y-1">
          <li><strong>Use spot instances:</strong> Save up to 80% on costs with RunPod spot pricing</li>
          <li><strong>Auto-shutdown:</strong> Configure auto-shutdown after inactivity to avoid unnecessary charges</li>
          <li><strong>Right-size your instance:</strong> Choose the minimum VRAM needed for your model</li>
          <li><strong>Batch processing:</strong> Process multiple translations in one session to maximize efficiency</li>
        </ul>
        <UAlert color="info" variant="soft" icon="i-lucide-info">
          <template #title>Persistent Storage</template>
          <template #description>
            Consider using RunPod's persistent storage to keep your models and avoid re-downloading them each time you start a new pod.
          </template>
        </UAlert>
      </div>
    </UCard>

    <UCard>
      <template #header>
        <div class="font-medium">Troubleshooting</div>
      </template>
      <div class="space-y-3 text-sm">
        <div>
          <div class="font-medium">Connection Issues</div>
          <ul class="list-disc pl-5 space-y-1 mt-1">
            <li>Ensure your pod is running and Ollama is started</li>
            <li>Check that the proxy URL is correct (replace {POD_ID})</li>
            <li>Wait 2-3 minutes after pod startup for Ollama to fully initialize</li>
          </ul>
        </div>
        <USeparator />
        <div>
          <div class="font-medium">Model Issues</div>
          <ul class="list-disc pl-5 space-y-1 mt-1">
            <li>Check if the model was successfully pulled (check logs)</li>
            <li>Ensure the model name matches exactly in LudoLingua settings</li>
            <li>Try pulling a smaller model if you're running out of VRAM</li>
          </ul>
        </div>
      </div>
    </UCard>
  </section>
</template>

<script setup lang="ts">
const { copy, copied } = useClipboard()

const containerCommand = ref(`bash -c "
apt update && apt install -y curl lshw &&
curl -fsSL https://ollama.com/install.sh | sh &&
nohup ollama serve > /root/ollama.log 2>&1 &
sleep 60 &&
ollama pull qwen2.5:14b &&
sleep infinity
"`)
</script>