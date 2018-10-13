void VirtManager::VirDomainBlockDeviceCreate(
    scaled::Result& result,
    const InternalSession& session,
    const scaled::VirDomainBlockDevice& device,
    const Forte::FString& desc,
    const std::vector<std::string>& descParams,
    const VirtDomainFacadePtr& vm)
{
    FTRACE;

    CGET("scaled.Configurator", Configurator, c);
    CGET("scaled.TaskManager", TaskManager, tm);
    CGET("scaled.ConfigDataModel", ConfigDataModel, config);

    VirtDomainFacadePtr vm(
        VirtDomainFacade::GetDomainHandle(device.virDomainUUID, mContext));

    FString desc("Create block device for Virtual Machine %@");
    std::vector<std::string> descParams;
    descParams.push_back(vm->GetName());
    TaskTagTransactionPtr transaction = tm.GetNewTaskTag(
        vm->GetUUID(), desc, session.GetUserUUID(), descParams);

    VirtDomainFacadePtr vm(
        VirtDomainFacade::GetDomainHandle(device.virDomainUUID, mContext));

    FString desc("Create block device for Virtual Machine %@");
    std::vector<std::string> descParams;
    descParams.push_back(vm->GetName());
    if (vm->GetState() != scaled::VirDomainState::SHUTOFF)
    {
        hlog_and_throw(HLOG_WARN,
                       EVirDomainNotShutOff(
                           vm->GetName()));
    }

    FString blockDeviceUUID;
    blockDeviceUUID = vm->CreateEntryInDataModelForBlockDevice(device);

    FStringVector args;
    args.push_back(device.virDomainUUID);
    args.push_back(blockDeviceUUID);

    enqueueCallbackTask(tm, transaction, CALLBACK_CREATE_BLOCKDEVICE, args);

    result.createdUUID = blockDeviceUUID;
    result.taskTag = FString(transaction->GetTaskTagID());
    transaction->Commit();
}
