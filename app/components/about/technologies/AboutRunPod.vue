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
          <div class="text-muted mb-2">Choose a GPU instance with sufficient VRAM and storage for the models:</div>
          <UAlert color="warning" variant="soft" icon="i-lucide-hard-drive" class="mb-3">
            <template #title>Storage Requirements</template>
            <template #description>
              <strong>Minimum 50GB storage</strong> required for both models (qwen2.5:14b ~20GB + qwen3:14b ~20GB + system overhead). 
              <br><br>
              <strong>Storage optimization:</strong> If you only need one model, you can remove one line from the container command and use <strong>30GB storage</strong> instead (single model ~20GB + system overhead).
            </template>
          </UAlert>
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
          <div class="text-muted mb-2">Use this command to automatically install and configure Ollama with the required models:</div>
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
          <UAlert color="info" variant="soft" icon="i-lucide-info" class="mt-2">
            <template #title>Pre-configured Models</template>
            <template #description>
              This command installs the models that LudoLingua supports: <code>qwen2.5:14b</code> and <code>qwen3:14b</code>. You'll select from these models in the LudoLingua settings.
              <br><br>
              <strong>Single model option:</strong> To save storage space, you can remove either <code>ollama pull qwen2.5:14b &&</code> or <code>ollama pull qwen3:14b &&</code> from the command above. This reduces storage requirement to 30GB.
            </template>
          </UAlert>
        </div>

        <USeparator />

        <div>
          <div class="font-medium mb-2">3. Single Model Setup (Optional)</div>
          <div class="text-muted mb-2">If you only need one model to save storage space, use one of these modified commands:</div>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
            <UCard class="bg-gray-50 dark:bg-gray-800">
              <div class="text-xs font-medium mb-2">Qwen2.5:14b only (30GB storage)</div>
              <div class="flex items-start justify-between gap-2">
                <pre class="text-xs overflow-x-auto flex-1"><code>{{ singleModelCommandQwen25 }}</code></pre>
                <UTooltip text="Copy to clipboard" :content="{ side: 'left' }">
                  <UButton
                    :color="copiedQwen25 ? 'success' : 'neutral'"
                    variant="link"
                    size="sm"
                    :icon="copiedQwen25 ? 'i-lucide-copy-check' : 'i-lucide-copy'"
                    aria-label="Copy to clipboard"
                    @click="copyQwen25(singleModelCommandQwen25)"
                  />
                </UTooltip>
              </div>
            </UCard>
            <UCard class="bg-gray-50 dark:bg-gray-800">
              <div class="text-xs font-medium mb-2">Qwen3:14b only (30GB storage)</div>
              <div class="flex items-start justify-between gap-2">
                <pre class="text-xs overflow-x-auto flex-1"><code>{{ singleModelCommandQwen3 }}</code></pre>
                <UTooltip text="Copy to clipboard" :content="{ side: 'left' }">
                  <UButton
                    :color="copiedQwen3 ? 'success' : 'neutral'"
                    variant="link"
                    size="sm"
                    :icon="copiedQwen3 ? 'i-lucide-copy-check' : 'i-lucide-copy'"
                    aria-label="Copy to clipboard"
                    @click="copyQwen3(singleModelCommandQwen3)"
                  />
                </UTooltip>
              </div>
            </UCard>
          </div>
        </div>

        <USeparator />

        <div>
          <div class="font-medium mb-2">4. Get Your Pod URL</div>
          <div class="text-muted mb-2">Your RunPod endpoint will be:</div>
          <UCard class="bg-gray-50 dark:bg-gray-800">
            <code class="text-sm">https://{POD_ID}-11434.proxy.runpod.net</code>
          </UCard>
          <div class="text-muted mt-1">Replace <code>{POD_ID}</code> with your actual pod ID from RunPod dashboard.</div>
        </div>

        <USeparator />

        <div>
          <div class="font-medium mb-2">5. Configure in LudoLingua</div>
          <div class="text-muted mb-2">In Settings, set:</div>
          <ul class="list-disc pl-5 space-y-1">
            <li><strong>Provider:</strong> RunPod</li>
            <li><strong>Base URL:</strong> <code>https://{POD_ID}-11434.proxy.runpod.net</code></li>
            <li><strong>Model:</strong> Select from available models (Qwen2.5 14B or qwen3 14b)</li>
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
              <div><strong>Storage:</strong> 50GB+ (recommended)</div>
              <div><strong>Models:</strong> Qwen2.5:14b, qwen3:14b</div>
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
              <div><strong>Storage:</strong> 50GB+ (minimum)</div>
              <div><strong>Models:</strong> Qwen2.5:14b, qwen3:14b (may require quantization)</div>
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
          <li><strong>Storage optimization:</strong> Use exactly 50GB for both models or 30GB for single model to minimize costs</li>
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
            <li>Check if both models (qwen2.5:14b and qwen3:14b) were successfully pulled</li>
            <li>Ensure the model name matches exactly in LudoLingua settings</li>
            <li>Both models should be available in the LudoLingua model dropdown</li>
          </ul>
        </div>
      </div>
    </UCard>
  </section>
</template>

<script setup lang="ts">
const { copy, copied } = useClipboard()
const { copy: copyQwen25, copied: copiedQwen25 } = useClipboard()
const { copy: copyQwen3, copied: copiedQwen3 } = useClipboard()

const containerCommand = ref(`bash -c "
apt update && apt install -y curl lshw &&
curl -fsSL https://ollama.com/install.sh | sh &&
nohup ollama serve > /root/ollama.log 2>&1 &
sleep 60 &&
ollama pull qwen2.5:14b-instruct-q5_K_M &&
ollama pull qwen3:14b-q4_K_M &&
sleep infinity
"`)

const singleModelCommandQwen25 = ref(`bash -c "
apt update && apt install -y curl lshw &&
curl -fsSL https://ollama.com/install.sh | sh &&
nohup ollama serve > /root/ollama.log 2>&1 &
sleep 60 &&
ollama pull qwen2.5:14b-instruct-q5_K_M &&
sleep infinity
"`)

const singleModelCommandQwen3 = ref(`bash -c "
apt update && apt install -y curl lshw &&
curl -fsSL https://ollama.com/install.sh | sh &&
nohup ollama serve > /root/ollama.log 2>&1 &
sleep 60 &&
ollama pull qwen3:14b-q4_K_M &&
sleep infinity
"`)
</script>