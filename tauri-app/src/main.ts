import { invoke } from "@tauri-apps/api/core";

// Interface para garantir que nosso objeto 'app' tenha os tipos corretos.
interface AppInfo {
  id: string | null;
  name: string;
  description: string | null;
  executable: string | null;
  icon: string | null;
}

// Garante que o código só rode depois que a página HTML for carregada.
window.addEventListener("DOMContentLoaded", () => {
  const appListEl = document.querySelector("#app-list") as HTMLUListElement;

  // Função para carregar e mostrar os apps.
  const loadAndDisplayApps = async () => {
    if (!appListEl) {
      console.error("Elemento #app-list não encontrado!");
      return;
    }

    appListEl.innerHTML = "<li>Carregando...</li>";

    try {
      // Chama o comando do backend Rust.
      const apps = await invoke<AppInfo[]>("list_installed_apps");

      appListEl.innerHTML = ""; // Limpa a lista

      if (apps.length === 0) {
        appListEl.innerHTML = "<li>Nenhuma aplicação encontrada.</li>";
        return;
      }

      // Cria os elementos da lista.
      apps.forEach(app => {
        const listItem = document.createElement("li");
        listItem.textContent = `${app.id} ${app.name} (${app.description || 'Sem descrição'})`;
        listItem.addEventListener("click", async () => {
          try {
            // Chama o comando para abrir o app.
            await invoke("launch_app", { appId: app.id });
          } catch (error) {
            console.error("Erro ao abrir a aplicação:", error);
            alert(`Erro ao abrir a aplicação: ${error}`);
          }
        });
        if (app.icon) {
          const iconImg = document.createElement("img");
          iconImg.src = app.icon;
          iconImg.alt = `${app.name} ícone`;
          iconImg.style.width = "24px"; // Define um tamanho fixo para o ícone
          iconImg.style.height = "24px"; // Define um tamanho fixo para o ícone
          listItem.prepend(iconImg); // Adiciona o ícone antes do texto
        }
        appListEl.appendChild(listItem);
      });

    } catch (error) {
      console.error("Erro ao carregar aplicações:", error);
      appListEl.innerHTML = `<li>Erro ao carregar: ${error}</li>`;
    }
  };

  // Chama a função assim que a página carregar.
  loadAndDisplayApps();
});