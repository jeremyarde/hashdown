import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/app-sidebar";
import { AppCrumbs } from "@/components/app-crumbs";

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <SidebarProvider>
      <AppSidebar />
      <main>
        <div className="flex flex-row items-center justify-between">
          <SidebarTrigger />
          <AppCrumbs />
        </div>
        {children}
      </main>
    </SidebarProvider>
  );
}
