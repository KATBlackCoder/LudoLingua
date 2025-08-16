<template>
  <section>
    <div class="flex items-center gap-2 mb-2">
      <h3 class="text-lg font-medium">Placeholders Reference</h3>
      <UBadge color="primary" variant="soft" size="xs">Do not edit inside [ … ]</UBadge>
    </div>

    <UAlert color="error" variant="soft" icon="i-heroicons-information-circle" class="mb-3">
      <template #title>Preserve placeholders exactly</template>
      <template #description>
        <div class="text-sm">
          Any token enclosed in square brackets (e.g., <code>[COLOR_5]</code>) is a placeholder that encodes engine semantics. Do not translate, change, remove, or re-order these unless you are sure you know what you are doing. They will be restored on export.
        </div>
      </template>
    </UAlert>

    <div class="space-y-4 text-sm">
      <UCard>
        <template #header>
          <span class="font-medium">Formatting & Names</span>
        </template>
        <ul class="list-disc pl-5 leading-relaxed">
          <li><code>[COLOR_n]</code> → <code>\C[n]</code> or <code>\c[n]</code> (color index)</li>
          <li><code>[NAME_n]</code> → <code>\N[n]</code> (name by index)</li>
          <li><code>[NEWLINE_n]</code> → <code>\n[n]</code> (insert n newlines)</li>
          <li><code>[ICON_n]</code> → <code>\i[n]</code> (icon index)</li>
          <li><code>[FONT_n]</code> → <code>\f[n]</code> (font/face index)</li>
          <li><code>[WOLF_END]</code> → <code>\E</code> (end/reset formatting)</li>
          <li><code>[RUBY_START]</code> → <code>\\r</code> (ruby marker start)</li>
        </ul>
      </UCard>

      <UCard>
        <template #header>
          <span class="font-medium">Variables, Items & Entities</span>
        </template>
        <ul class="list-disc pl-5 leading-relaxed">
          <li><code>[VARIABLE_…]</code>/<code>[variable_…]</code> → <code>\V[</code>/<code>\v[</code> (variables)</li>
          <li><code>[SWITCH_…]</code> → <code>\S[</code> (switches)</li>
          <li><code>[ITEM_…]</code>/<code>[WEAPON_…]</code>/<code>[ARMOR_…]</code> → <code>\I[</code>/<code>\W[</code>/<code>\A[</code></li>
          <li><code>[ACTOR_…]</code> → <code>\P[</code> (actor ref)</li>
          <li><code>[GOLD]</code>/<code>[CURRENCY]</code> → <code>\G</code>/<code>\$</code></li>
          <li><code>[AT_n]</code> → <code>@n</code> (event/command marker)</li>
          <li><code>[SLOT_xxx]</code> → <code>\s[n]</code> (slot ref; preserves leading zeros)</li>
          <li><code>[CSELF_n]</code> → <code>\cself[n]</code> (self color)</li>
        </ul>
      </UCard>

      <UCard>
        <template #header>
          <span class="font-medium">Control & Flow</span>
        </template>
        <ul class="list-disc pl-5 leading-relaxed">
          <li><code>[CTRL_DOT]</code>/<code>[CTRL_WAIT]</code>/<code>[CTRL_INSTANT]</code>/<code>[CTRL_INPUT]</code> → <code>\.</code>/<code>\|</code>/<code>\^</code>/<code>\!</code></li>
          <li><code>[CONDITIONAL_vX&gt;Y]</code> → <code>en(v[X]&gt;Y)</code></li>
          <li><code>[NEWLINE]</code> → actual newline <code>\n</code></li>
          <li><code>[CARRIAGE_RETURN]</code> → <code>\r</code></li>
        </ul>
      </UCard>

      <UCard>
        <template #header>
          <span class="font-medium">Arguments & Whitespace</span>
        </template>
        <ul class="list-disc pl-5 leading-relaxed">
          <li><code>[ARG_n]</code> → <code>%n</code> (argument placeholder)</li>
          <li><code>[NUM_PREFIX_ddd]</code> → <code>ddd＿</code> (numeric prefix token)</li>
          <li><code>[FWSPC_n]</code>/<code>[SPC_n]</code>/<code>[TAB_n]</code> → n full‑width spaces / n ASCII spaces / n tabs</li>
        </ul>
      </UCard>
    </div>
  </section>
</template>


