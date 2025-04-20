<section>
  <!-- Tab Content (di atas) -->
  <div class="tab-content bg-danger" id="myTabContent">
    <div
      class="tab-pane fade"
      id="home-tab-pane"
      role="tabpanel"
      aria-labelledby="home-tab"
      tabindex="0"
    >
      Home content
    </div>
    <div
      class="tab-pane fade"
      id="profile-tab-pane"
      role="tabpanel"
      aria-labelledby="profile-tab"
      tabindex="0"
    >
      Profile content
    </div>
    <div
      class="tab-pane fade"
      id="contact-tab-pane"
      role="tabpanel"
      aria-labelledby="contact-tab"
      tabindex="0"
    >
      Contact content
    </div>
    <div
      class="tab-pane fade"
      id="disabled-tab-pane"
      role="tabpanel"
      aria-labelledby="disabled-tab"
      tabindex="0"
    >
      Disabled content
    </div>
  </div>

  <!-- Tabs (di bawah) -->
  <ul class="nav nav-tabs bottom-tab" id="myTab" role="tablist">
    <li class="nav-item" role="presentation">
      <button
        class="nav-link"
        id="home-tab"
        data-bs-toggle="tab"
        data-bs-target="#home-tab-pane"
        type="button"
        role="tab"
        aria-controls="home-tab-pane"
        aria-selected="true">Home</button
      >
    </li>
    <li class="nav-item" role="presentation">
      <button
        class="nav-link"
        id="profile-tab"
        data-bs-toggle="tab"
        data-bs-target="#profile-tab-pane"
        type="button"
        role="tab"
        aria-controls="profile-tab-pane"
        aria-selected="false">Profile</button
      >
    </li>
    <li class="nav-item" role="presentation">
      <button
        class="nav-link"
        id="contact-tab"
        data-bs-toggle="tab"
        data-bs-target="#contact-tab-pane"
        type="button"
        role="tab"
        aria-controls="contact-tab-pane"
        aria-selected="false">Contact</button
      >
    </li>
    <li class="nav-item" role="presentation">
      <button
        class="nav-link"
        id="disabled-tab"
        data-bs-toggle="tab"
        data-bs-target="#disabled-tab-pane"
        type="button"
        role="tab"
        aria-controls="disabled-tab-pane"
        aria-selected="false">Disabled</button
      >
    </li>
  </ul>
</section>

<style>
  .tab-content {
    padding: 1rem;
    margin-bottom: 4rem; /* supaya tidak ketimpa tab */
  }

  .bottom-tab {
    position: fixed;
    bottom: 1rem;
    left: 2rem;
    right: 2rem;
    background-color: white;
    z-index: 999;
    border-radius: 10px;
    overflow: hidden;
  }
</style>
