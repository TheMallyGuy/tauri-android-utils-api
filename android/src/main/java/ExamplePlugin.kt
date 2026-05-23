package com.plugin.androidutils

import android.content.pm.ApplicationInfo
import android.content.pm.PackageManager
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.JSArray
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import android.app.Activity

@TauriPlugin
class AndroidUtils(private val activity: Activity) : Plugin(activity) {

    @Command
    fun getInstalledApps(invoke: Invoke) {
        val pm: PackageManager = activity.packageManager
        val packages = pm.getInstalledApplications(PackageManager.GET_META_DATA)
        val result = JSArray()

        for (appInfo in packages) {
            val app = JSObject()
            app.put("packageName", appInfo.packageName)
            app.put("appName", pm.getApplicationLabel(appInfo).toString())
            app.put("isSystemApp", (appInfo.flags and ApplicationInfo.FLAG_SYSTEM) != 0)
            app.put("isUserApp", (appInfo.flags and ApplicationInfo.FLAG_SYSTEM) == 0)

            try {
                val pkgInfo = pm.getPackageInfo(appInfo.packageName, 0)
                app.put("versionName", pkgInfo.versionName ?: "")
                app.put("versionCode", pkgInfo.longVersionCode)
                app.put("firstInstallTime", pkgInfo.firstInstallTime)
                app.put("lastUpdateTime", pkgInfo.lastUpdateTime)
            } catch (e: PackageManager.NameNotFoundException) {
                app.put("versionName", "")
                app.put("versionCode", 0)
            }

            result.put(app)
        }

        val response = JSObject()
        response.put("apps", result)
        invoke.resolve(response)
    }

    @Command
    fun getUserInstalledApps(invoke: Invoke) {
        val pm: PackageManager = activity.packageManager
        val packages = pm.getInstalledApplications(PackageManager.GET_META_DATA)
        val result = JSArray()

        for (appInfo in packages) {
            val isUserApp = (appInfo.flags and ApplicationInfo.FLAG_SYSTEM) == 0
            val isUpdatedSystemApp = (appInfo.flags and ApplicationInfo.FLAG_UPDATED_SYSTEM_APP) != 0

            if (isUserApp || isUpdatedSystemApp) {
                val app = JSObject()
                app.put("packageName", appInfo.packageName)
                app.put("appName", pm.getApplicationLabel(appInfo).toString())
                app.put("isSystemApp", !isUserApp)
                app.put("isUserApp", isUserApp)

                try {
                    val pkgInfo = pm.getPackageInfo(appInfo.packageName, 0)
                    app.put("versionName", pkgInfo.versionName ?: "")
                    app.put("versionCode", pkgInfo.longVersionCode)
                    app.put("firstInstallTime", pkgInfo.firstInstallTime)
                    app.put("lastUpdateTime", pkgInfo.lastUpdateTime)
                } catch (e: PackageManager.NameNotFoundException) {
                    app.put("versionName", "")
                    app.put("versionCode", 0)
                }

                result.put(app)
            }
        }

        val response = JSObject()
        response.put("apps", result)
        invoke.resolve(response)
    }
}
